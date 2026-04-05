#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri::WebviewWindowBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    api_key: Option<String>,
    region: Option<String>,
    use_edge: Option<bool>,
    use_azure: Option<bool>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: None,
            region: Some("eastus".to_string()),
            use_edge: Some(true),
            use_azure: Some(true),
        }
    }
}

// 检查是否是新安装（通过版本标记文件）
fn is_new_installation(config_dir: &PathBuf) -> bool {
    const CURRENT_VERSION: &str = "1.0.0";
    let version_file = config_dir.join(".version");
    
    if !version_file.exists() {
        return true;
    }
    
    match std::fs::read_to_string(&version_file) {
        Ok(version) => version.trim() != CURRENT_VERSION,
        Err(_) => true,
    }
}

// 标记已安装版本
fn mark_installed(config_dir: &PathBuf) -> Result<(), String> {
    const CURRENT_VERSION: &str = "1.0.0";
    let version_file = config_dir.join(".version");
    std::fs::write(&version_file, CURRENT_VERSION).map_err(|e| e.to_string())
}

fn get_settings(app: &AppHandle) -> Settings {
    let config_dir = app.path().app_config_dir().unwrap_or_else(|_| PathBuf::from("."));
    let settings_path = config_dir.join("settings.json");
    
    log::info!("Loading settings from: {}", settings_path.display());
    
    // 如果是新安装，清空 API Key 但保留其他设置
    if is_new_installation(&config_dir) {
        log::info!("New installation detected, clearing API key for security");
        
        // 尝试读取旧设置
        let mut settings = if settings_path.exists() {
            match std::fs::read_to_string(&settings_path) {
                Ok(settings_str) => {
                    serde_json::from_str::<Settings>(&settings_str).unwrap_or_default()
                }
                Err(_) => Settings::default(),
            }
        } else {
            Settings::default()
        };
        
        // 清空 API Key（安全考虑）
        settings.api_key = None;
        
        // 保存清理后的设置
        let _ = save_settings_to_file(app, &settings);
        let _ = mark_installed(&config_dir);
        
        return settings;
    }
    
    let settings_str = std::fs::read_to_string(&settings_path).unwrap_or_default();
    if settings_str.is_empty() {
        log::warn!("Settings file not found or empty, using defaults");
        return Settings::default();
    }
    
    match serde_json::from_str::<Settings>(&settings_str) {
        Ok(settings) => {
            let api_key_nonempty = settings.api_key.as_ref().map(|k| !k.is_empty()).unwrap_or(false);
    log::info!("Settings loaded: api_key exists={}, use_edge={:?}", 
                api_key_nonempty,
                settings.use_edge);
            settings
        }
        Err(e) => {
            log::error!("Failed to parse settings: {}, using defaults", e);
            Settings::default()
        }
    }
}

fn save_settings_to_file(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    
    // 确保配置目录存在
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }
    
    let settings_path = config_dir.join("settings.json");
    let settings_json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(&settings_path, settings_json).map_err(|e| e.to_string())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

#[cfg(windows)]
fn run_hidden_command(script: &str) -> Result<String, String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to run command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    log::debug!("Command stdout: {}", stdout);
    if !stderr.is_empty() {
        log::debug!("Command stderr: {}", stderr);
    }

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!("Command failed with exit code: {:?}, stderr: {}", output.status.code(), stderr))
    }
}

#[cfg(not(windows))]
fn run_hidden_command(script: &str) -> Result<String, String> {
    use std::process::Command;
    let output = Command::new("sh")
        .args(["-c", script])
        .output()
        .map_err(|e| format!("Failed to run command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// 从文本中提取 vol 标签的值，返回第一个找到的值（用于 ffmpeg 全局音量调整）
fn extract_vol_value(text: &str) -> Option<f64> {
    let re = regex::Regex::new(r"\[\[vol\s+([\d.]+)\]\]").unwrap();
    re.captures(text)
        .and_then(|caps| caps[1].parse().ok())
}

// 使用 ffmpeg 调整音频音量
fn apply_ffmpeg_volume(audio_path: &PathBuf, vol: f64) -> Result<(), String> {
    let ffmpeg_path = "D:\\tools\\ffmpeg\\bin\\ffmpeg.exe";
    let input_path = audio_path.to_string_lossy();

    // 写标记文件到桌面证明代码被执行了
    let desktop = dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("."));
    let marker_path = desktop.join("ffmpeg_called.txt");
    let _ = std::fs::write(&marker_path, format!("vol={}, input={}, time={}", vol, input_path, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));

    // 先写到临时文件，避免读写同一文件冲突
    let temp_path = format!("{}.voltmp", input_path);

    // 运行 ffmpeg volume 滤镜
    let ps_script = format!(
        r#"& "{}" -hide_banner -y -i "{}" -af "volume={}" -ar 24000 -ac 1 "{}" 2>&1"#,
        ffmpeg_path, input_path, vol, temp_path
    );

    log::info!("Running ffmpeg: input={}, vol={}, temp={}", input_path, vol, temp_path);

    let result = run_hidden_command(&ps_script)?;

    if result.contains("Error") || result.contains("error opening") {
        let _ = std::fs::remove_file(&temp_path);
        Err(format!("FFmpeg volume failed: {}", result))
    } else {
        // 用临时文件替换原文件
        if let Err(e) = std::fs::rename(&temp_path, audio_path) {
            std::fs::copy(&temp_path, audio_path).map_err(|e| format!("Copy failed: {}", e))?;
            let _ = std::fs::remove_file(&temp_path);
        }
        log::info!("FFmpeg volume adjusted: {}x", vol);
        Ok(())
    }
}

#[tauri::command]
async fn generate_speech(
    app: AppHandle,
    text: String,
    filename: String,
    voice: String,
    velocity: f64,
    save_path: String,
) -> Result<ApiResponse, String> {
    log::info!("Generating speech: voice={}, velocity={}, savePath='{}'", voice, velocity, save_path);
    log::info!("Received save_path length: {}, content: {:?}", save_path.len(), save_path);

    if text.trim().is_empty() {
        return Ok(ApiResponse {
            success: false,
            message: Some("Please enter some text".to_string()),
            path: None,
        });
    }

    let save_dir = if !save_path.is_empty() {
        PathBuf::from(&save_path)
    } else {
        dirs::download_dir().unwrap_or_else(|| PathBuf::from("."))
    };
    
    // 确保保存目录存在
    if !save_dir.exists() {
        log::info!("Creating save directory: {}", save_dir.display());
        std::fs::create_dir_all(&save_dir)
            .map_err(|e| format!("Failed to create save directory: {}", e))?;
    }
    
    let settings = get_settings(&app);
    let api_key = settings.api_key.unwrap_or_default();
    let region = settings.region.unwrap_or_else(|| "eastus".to_string());
    let mut use_azure = settings.use_azure.unwrap_or(true);
    let mut use_edge = settings.use_edge.unwrap_or(true);

    // 如果两个 TTS 引擎都被禁用了（保底），自动启用 Edge TTS
    if !use_azure && !use_edge {
        log::warn!("Both TTS engines disabled, falling back to Edge TTS");
        use_edge = true;
    }

    let output_path = save_dir.join(format!("{}.wav", filename));

    // 转换 SSML 符号
    let ssml_text = convert_ssml_markers(&text);
    log::info!("SSML converted text length: {}", ssml_text.len());

    // 检测是否包含 SSML 标签（表示用户使用了高级功能）
    let has_ssml = ssml_text.contains('<') && ssml_text.contains("break") || ssml_text.contains("prosody");

    // SSML 标签检测：Azure TTS 支持 inline prosody，Edge TTS 不支持（会产生嵌套无效标签）
    // 所以有 SSML 标签时优先使用 Azure TTS
    if has_ssml {
        log::info!("SSML tags detected, will use Azure TTS for proper support");
    }

    // 先尝试 Azure TTS（当开关开启、有 API Key 时优先使用，因为支持 SSML）
    if use_azure && !api_key.is_empty() {
        log::info!("Using Azure TTS");
        match call_azure_tts(&ssml_text, &voice, velocity, &output_path, &api_key, &region) {
            Ok(_) => {
                log::info!("Audio saved to: {}", output_path.display());
                // 检查是否有 vol 标签，用 ffmpeg 调整音量
                if let Some(vol) = extract_vol_value(&text) {
                    log::info!("Detected vol tag: {}, applying ffmpeg volume", vol);
                    if let Err(e) = apply_ffmpeg_volume(&output_path, vol) {
                        log::warn!("FFmpeg volume adjustment failed: {}", e);
                        // 继续返回成功，音频已生成只是音量没调而已
                    }
                }
                return Ok(ApiResponse {
                    success: true,
                    message: Some("Speech generated successfully!".to_string()),
                    path: Some(output_path.to_string_lossy().to_string()),
                });
            }
            Err(e) => {
                log::warn!("Azure TTS failed: {}, trying Edge TTS...", e);
                if !use_edge {
                    return Ok(ApiResponse {
                        success: false,
                        message: Some(e),
                        path: None,
                    });
                }
            }
        }
    } else if !use_azure {
        log::info!("Azure TTS disabled by user, using Edge TTS only");
    } else {
        log::warn!("Azure API key is empty, using Edge TTS");
    }

    // 检查 Edge TTS 开关
    if !use_edge {
        return Ok(ApiResponse {
            success: false,
            message: Some("Edge TTS is disabled in settings. Please enable at least one TTS engine.".to_string()),
            path: None,
        });
    }

    // Edge TTS
    match call_edge_tts(&ssml_text, &voice, velocity, &output_path) {
        Ok(_) => {
            log::info!("Edge TTS audio saved to: {}", output_path.display());
            // 检查是否有 vol 标签，用 ffmpeg 调整音量
            if let Some(vol) = extract_vol_value(&text) {
                log::info!("Detected vol tag: {}, applying ffmpeg volume", vol);
                if let Err(e) = apply_ffmpeg_volume(&output_path, vol) {
                    log::warn!("FFmpeg volume adjustment failed: {}", e);
                }
            }
            Ok(ApiResponse {
                success: true,
                message: Some("Speech generated successfully! (via Edge TTS)".to_string()),
                path: Some(output_path.to_string_lossy().to_string()),
            })
        }
        Err(e) => {
            log::error!("Edge TTS failed: {}", e);
            let error_msg = if !api_key.is_empty() {
                format!("Azure TTS failed, Edge TTS also failed: {}", e)
            } else {
                format!("Edge TTS failed (no Azure API key configured): {}", e)
            };
            Ok(ApiResponse {
                success: false,
                message: Some(error_msg),
                path: None,
            })
        }
    }
}

// 转换 SSML 标记到真正的 SSML
// [[slnc X]] -> <break time="Xms"/>
// [[rate X]]text[[/rate]] -> <prosody rate="X">text</prosody>
// [[vol X]]text[[/vol]] -> <prosody volume="X">text</prosody>
fn convert_ssml_markers(text: &str) -> String {
    let mut result = text.to_string();

    // 转换停顿标记 [[slnc X]] -> <break time="Xs" /> (edge-tts 兼容格式)
    let re_slnc = regex::Regex::new(r"\[\[slnc\s+(\d+)\]\]").unwrap();
    result = re_slnc.replace_all(&result, |caps: &regex::Captures| {
        let ms: i32 = caps[1].parse().unwrap_or(500);
        if ms >= 1000 {
            format!("<break time='{}s' />", ms / 1000)
        } else {
            format!("<break time='{}ms' />", ms)
        }
    }).to_string();

    // 转换语速标记 [[rate X]]text[[/rate]] -> <prosody rate="+X%">text</prosody>
    // rate 值转换为百分比: 0.8 -> "-20%", 1.2 -> "+20%"
    let re_rate = regex::Regex::new(r"\[\[rate\s+([\d.]+)\]\](.*?)\[\[/rate\]\]").unwrap();
    result = re_rate.replace_all(&result, |caps: &regex::Captures| {
        let rate: f64 = caps[1].parse().unwrap_or(1.0);
        let content = &caps[2];
        let percent = ((rate - 1.0) * 100.0).round() as i32;
        let rate_str = if percent >= 0 {
            format!("+{}%", percent)
        } else {
            format!("{}%", percent)
        };
        format!("<prosody rate='{}'>{}</prosody>", rate_str, content)
    }).to_string();

    // 转换音量标记 [[vol X]]text[[/vol]] -> <prosody volume="+X%">text</prosody>
    // vol 值转换为相对百分比: 0.5 -> "-50%", 1.0 -> "+0%", 1.5 -> "+50%", 2.0 -> "+100%"
    let re_vol = regex::Regex::new(r"\[\[vol\s+([\d.]+)\]\](.*?)\[\[/vol\]\]").unwrap();
    result = re_vol.replace_all(&result, |caps: &regex::Captures| {
        let vol: f64 = caps[1].parse().unwrap_or(1.0);
        let content = &caps[2];
        // Azure prosody volume: 0-100 绝对值，或 +X%/-X% 相对值
        // 转换为 50-200 的绝对值试试
        let vol_val = (vol * 100.0).round() as i32;
        let vol_clamped = vol_val.min(200).max(0);
        format!("<prosody volume='{}'>{}</prosody>", vol_clamped, content)
    }).to_string();

    result
}

fn call_azure_tts(text: &str, voice: &str, rate: f64, output_path: &PathBuf, api_key: &str, region: &str) -> Result<(), String> {
    let rate_percent = ((rate - 1.0) * 100.0).round() as i32;
    let rate_str = if rate_percent >= 0 {
        format!("+{}%", rate_percent)
    } else {
        format!("{}%", rate_percent)
    };

    // 根据音色名称提取 BCP-47 语言标签
    let lang_tag = if voice.starts_with("zh-CN") {
        "zh-CN"
    } else if voice.starts_with("zh-TW") {
        "zh-TW"
    } else if voice.starts_with("zh-HK") {
        "zh-HK"
    } else if voice.starts_with("en-US") {
        "en-US"
    } else if voice.starts_with("en-GB") {
        "en-GB"
    } else if voice.starts_with("en-AU") {
        "en-AU"
    } else if voice.starts_with("en-CA") {
        "en-CA"
    } else if voice.starts_with("en-IN") {
        "en-IN"
    } else if voice.starts_with("de-") {
        "de-DE"
    } else if voice.starts_with("fr-FR") {
        "fr-FR"
    } else if voice.starts_with("fr-CA") {
        "fr-CA"
    } else if voice.starts_with("es-ES") || voice.starts_with("es-MX") {
        "es-ES"
    } else if voice.starts_with("it-IT") {
        "it-IT"
    } else if voice.starts_with("ja-JP") {
        "ja-JP"
    } else if voice.starts_with("ko-KR") {
        "ko-KR"
    } else if voice.starts_with("pt-BR") {
        "pt-BR"
    } else if voice.starts_with("pt-PT") {
        "pt-PT"
    } else {
        // 默认使用 en-US
        "en-US"
    };

    // 检测是否有内联 prosody 标签（会导致嵌套问题）
    let has_inline_prosody = text.contains("<prosody");
    let escaped_text = text.replace("\n", " ");

    // 如果有内联 prosody，不能再用外层 prosody 包装，否则会嵌套无效
    let ssml = if has_inline_prosody {
        format!(r#"<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='{}'>
<voice name='{}'>
{}
</voice>
</speak>"#, lang_tag, voice, escaped_text)
    } else {
        format!(r#"<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='{}'>
<voice name='{}'>
<prosody rate='{}'>
{}
</prosody>
</voice>
</speak>"#, lang_tag, voice, rate_str, escaped_text)
    };

    // 使用 Python urllib 调用 Azure TTS（更可靠）
    let escaped_ssml = ssml.replace("'", "\\'");
    let escaped_ssml = escaped_ssml.replace("\n", " ");

    let python_script = format!(r#"
import urllib.request
import urllib.error
import sys

headers = {{
    'Ocp-Apim-Subscription-Key': '{}',
    'Content-Type': 'application/ssml+xml',
    'X-Microsoft-OutputFormat': 'riff-24khz-16bit-mono-pcm'
}}

uri = 'https://{}.tts.speech.microsoft.com/cognitiveservices/v1'

try:
    req = urllib.request.Request(uri, data='{}'.encode('utf-8'), headers=headers, method='POST')
    response = urllib.request.urlopen(req, timeout=30)
    with open('{}', 'wb') as f:
        f.write(response.read())
    print('AZURE_SUCCESS')
except urllib.error.HTTPError as e:
    error_body = e.read().decode('utf-8', errors='replace')[:200]
    print(f'AZURE_ERROR:HTTP{{e.code}}:{{error_body}}')
    sys.exit(1)
except Exception as e:
    print(f'AZURE_ERROR:{{e}}')
    sys.exit(1)
"#, api_key, region, escaped_ssml, output_path.to_string_lossy().replace("\\", "\\\\"));

    let python_exe = get_python_exe_path()?;

    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = std::process::Command::new(&python_exe)
        .args(["-c", &python_script])
        .creation_flags(CREATE_NO_WINDOW)
        .current_dir(std::env::temp_dir())
        .output()
        .map_err(|e| format!("Failed to run Python: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() || !stdout.contains("AZURE_SUCCESS") {
        log::error!("Azure TTS error: {} {}", stdout, stderr);
        return Err(format!("Azure TTS failed: {} {}", stdout.trim(), stderr.trim()));
    }

    log::info!("Azure TTS succeeded");
    Ok(())
}

// 获取打包的 Python 解释器路径
fn get_python_exe_path() -> Result<PathBuf, String> {
    let exe_path = std::env::current_exe().map_err(|e| format!("Cannot get exe path: {}", e))?;
    let exe_dir = exe_path.parent().ok_or("Cannot get exe directory")?;
    log::info!("Exe path: {}, dir: {}", exe_path.display(), exe_dir.display());

    // 候选路径列表 - 根据实际打包结构调整
    let candidates = vec![
        // NSIS 安装包：exe 同级目录下的 python（实际观察到的位置）
        exe_dir.join("python").join("python.exe"),
        // Tauri resources 目录（某些打包方式）
        exe_dir.join("resources").join("python").join("python.exe"),
        // 开发模式：src-tauri/python
        exe_dir.join("..").join("..").join("python").join("python.exe"),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            log::info!("Found Python at: {}", candidate.display());
            return Ok(candidate.clone());
        }
    }

    // 动态扫描 exe 同级目录下所有子目录，查找 python.exe
    if let Ok(entries) = std::fs::read_dir(exe_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let python_exe = path.join("python.exe");
                if python_exe.exists() {
                    log::info!("Found Python via dynamic scan at: {}", python_exe.display());
                    return Ok(python_exe);
                }
            }
        }
    }

    // 打包后必须找到嵌入式 Python，不再回退到系统 Python
    let tried_paths: Vec<String> = candidates.iter().map(|p| p.display().to_string()).collect();
    Err(format!("未找到内置 Python，请重新安装 G-Reader。尝试过的路径: {}", tried_paths.join(", ")))
}

fn call_edge_tts(text: &str, voice: &str, rate: f64, output_path: &PathBuf) -> Result<(), String> {
    use std::process::Command;
    #[cfg(windows)]
    use std::os::windows::process::CommandExt;
    #[cfg(windows)]
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let rate_percent = ((rate - 1.0) * 100.0).round() as i32;
    let rate_str = if rate_percent >= 0 {
        format!("+{}%", rate_percent)
    } else {
        format!("{}%", rate_percent)
    };

    // 获取打包的 Python 解释器
    let python_exe = get_python_exe_path()
        .map_err(|e| format!("Python not found: {}. Please reinstall G-Reader.", e))?;
    log::info!("Using Python: {}", python_exe.display());

    // 将文本写入临时文件以避免命令行编码问题
    let temp_dir = std::env::temp_dir();
    // 使用时间戳+进程ID+随机数确保唯一性
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    // 使用 rand crate 生成随机数确保文件名唯一
    let random_id: u32 = rand::Rng::gen(&mut rand::thread_rng());
    let temp_text_file = temp_dir.join(format!("greader_text_{}_{}_{}.txt", std::process::id(), timestamp, random_id));
    std::fs::write(&temp_text_file, text)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    // 确保临时文件在函数结束时被清理（使用 RAII 模式）
    struct TempFileCleanup(PathBuf);
    impl Drop for TempFileCleanup {
        fn drop(&mut self) {
            if self.0.exists() {
                if let Err(e) = std::fs::remove_file(&self.0) {
                    log::warn!("Failed to clean temp file {}: {}", self.0.display(), e);
                } else {
                    log::debug!("Temp file cleaned: {}", self.0.display());
                }
            }
        }
    }
    let _temp_file_cleanup = TempFileCleanup(temp_text_file.clone());

    // 获取路径字符串
    let temp_text_path = temp_text_file.to_string_lossy().to_string();
    let output_path_str = output_path.to_string_lossy().to_string();

    // 构建 Python 脚本
    let py_script = format!(
        "import asyncio, edge_tts\nimport sys\nimport traceback\n\ntry:\n    text = open(r'{}', 'r', encoding='utf-8').read()\n    \n    async def main():\n        c = edge_tts.Communicate(text, '{}', rate='{}')\n        await c.save(r'{}')\n    \n    asyncio.run(main())\n    print('EDGE_TTS_SUCCESS')\n    \nexcept Exception as e:\n    print('EDGE_TTS_ERROR:' + str(e))\n    traceback.print_exc()\n    sys.exit(1)",
        temp_text_path, voice, rate_str, output_path_str
    );

    log::info!("Running Edge TTS: voice={}, output={}", voice, output_path_str);

    // 直接调用 Python，不通过 PowerShell
    let mut cmd = Command::new(&python_exe);
    cmd.args(["-c", &py_script]);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd.output()
        .map_err(|e| format!("Failed to run Python '{}': {}. Check if antivirus blocked it.", python_exe.display(), e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // 临时文件会在 _temp_file_cleanup 的 Drop 实现中自动清理

    if output.status.success() {
        if stdout.trim().contains("EDGE_TTS_SUCCESS") {
            if output_path.exists() {
                match std::fs::metadata(output_path) {
                    Ok(metadata) if metadata.len() > 0 => {
                        log::info!("Edge TTS success: {} ({} bytes)", output_path.display(), metadata.len());
                        return Ok(());
                    }
                    Ok(_) => {
                        let msg = "Output file was created but is empty".to_string();
                        log::error!("{}", msg);
                        return Err(msg);
                    }
                    Err(e) => {
                        let msg = format!("Failed to verify output file: {}", e);
                        log::error!("{}", msg);
                        return Err(msg);
                    }
                }
            } else {
                let msg = "Output file was not created".to_string();
                log::error!("{}", msg);
                return Err(msg);
            }
        } else {
            // Python 脚本执行成功但没有输出成功标志
            let msg = format!("Edge TTS script executed but failed. stdout: {}, stderr: {}", stdout, stderr);
            log::error!("{}", msg);
            return Err(msg);
        }
    } else {
        // Python 脚本执行失败
        let error_detail = if stderr.contains("EDGE_TTS_ERROR:") {
            stderr.split("EDGE_TTS_ERROR:").last().unwrap_or("Unknown error").trim().to_string()
        } else if !stderr.is_empty() {
            stderr.trim().to_string()
        } else if !stdout.is_empty() {
            stdout.trim().to_string()
        } else {
            format!("Python exited with code {:?}", output.status.code())
        };
        
        let msg = format!("Edge TTS failed: {}", error_detail);
        log::error!("{}", msg);
        Err(msg)
    }
}

#[tauri::command]
async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    // 如果设置窗口已存在，则显示并聚焦
    if let Some(win) = app.get_webview_window("settings") {
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    
    // 创建新的设置窗口
    WebviewWindowBuilder::new(&app, "settings", tauri::WebviewUrl::App("settings.html".into()))
        .title("G-Reader Settings")
        .inner_size(500.0, 420.0)
        .min_inner_size(400.0, 350.0)
        .resizable(true)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .center()
        .build()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: String) -> Result<(), String> {
    let settings: Settings = serde_json::from_str(&settings).map_err(|e| e.to_string())?;
    save_settings_to_file(&app, &settings)
}

#[tauri::command]
fn get_settings_command(app: AppHandle) -> Result<Settings, String> {
    Ok(get_settings(&app))
}

// 验证 Azure API Key 是否有效
// 通过调用 Azure 的语音列表 API 来测试
#[tauri::command]
async fn validate_azure_api_key(api_key: String, region: String) -> Result<bool, String> {
    if api_key.is_empty() {
        return Ok(false);
    }

    let check_script = format!(
        r#"
$headers = @{{
    'Ocp-Apim-Subscription-Key' = '{}'
}}

try {{
    $response = Invoke-RestMethod -Uri 'https://{}.tts.speech.microsoft.com/cognitiveservices/voices/list' -Headers $headers -Method Get -TimeoutSec 10
    Write-Output 'VALID'
}} catch {{
    Write-Output 'INVALID'
}}
"#,
        api_key.replace("'", "''"),
        region
    );

    match run_hidden_command(&check_script) {
        Ok(output) => {
            let is_valid = output.trim() == "VALID";
            log::info!("Azure API Key validation result: {}", if is_valid { "VALID" } else { "INVALID" });
            Ok(is_valid)
        }
        Err(e) => {
            log::warn!("Azure API Key validation failed: {}", e);
            Ok(false)
        }
    }
}

// 从 PDF 文件提取文本内容
#[tauri::command]
async fn extract_pdf_text(path: String) -> Result<String, String> {
    log::info!("Extracting text from PDF: {}", path);

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err("File not found".to_string());
    }

    match pdf_extract::extract_text(&path) {
        Ok(text) => {
            log::info!("PDF text extracted successfully, length: {}", text.len());
            Ok(text)
        }
        Err(e) => {
            log::error!("Failed to extract PDF text: {}", e);
            Err(format!("Failed to extract PDF text: {}. Make sure the PDF is not scanned/image-based.", e))
        }
    }
}

// 读取音频文件并返回 base64 编码的数据 URL
#[tauri::command]
async fn read_audio_file(path: String) -> Result<String, String> {
    log::info!("Reading audio file: {}", path);

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err("File not found".to_string());
    }

    let extension = path_buf
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mime_type = match extension.as_str() {
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "m4a" => "audio/mp4",
        "flac" => "audio/flac",
        "aac" => "audio/aac",
        "wma" => "audio/x-ms-wma",
        _ => "application/octet-stream",
    };

    let file_data = std::fs::read(&path_buf).map_err(|e| {
        log::error!("Failed to read audio file: {}", e);
        format!("Failed to read audio file: {}", e)
    })?;

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&file_data);
    let data_url = format!("data:{};base64,{}", mime_type, base64_data);

    log::info!("Audio file loaded successfully: {} ({} bytes)", path, file_data.len());
    Ok(data_url)
}

// 生成试听音频（不保存到磁盘，直接返回 base64 数据 URL）
#[tauri::command]
async fn preview_voice(voice: String, use_edge: bool, api_key: Option<String>, region: Option<String>) -> Result<String, String> {
    // Edge-only voices that don't exist in Azure - map to Azure equivalents
    let azure_voice = if voice.contains("Laura") {
        "es-ES-NuriaNeural".to_string()
    } else if voice.contains("Alvaro") {
        "es-ES-AlvaroNeural".to_string()
    } else if voice.contains("Elvira") {
        "es-ES-ElviraNeural".to_string()
    } else if voice.contains("Helena") {
        "es-MX-DaliaNeural".to_string()
    } else if voice.contains("Diego") {
        "es-MX-JorgeNeural".to_string()
    } else {
        voice.clone()
    };

    // 根据音色名称推断语言，选择对应的预览文本
    let preview_text = if voice.starts_with("zh-CN") || voice.starts_with("zh-TW") || voice.starts_with("zh-HK") {
        // 中文音色
        "你好，这里是音色测试，你可以通过试听选择自己喜欢的音色。"
    } else if voice.starts_with("de-") {
        // 德语音色
        "Hallo, dies ist ein Stimmtest. Sie können ihn nutzen, um Ihre bevorzugte Stimme auszuwählen."
    } else if voice.starts_with("ja-") {
        // 日语音色
        "こんにちは、これは音声テストです。聴いて、好みの音声を選ぶことができます。"
    } else if voice.starts_with("ko-") {
        // 韩语音色
        "안녕하세요, 이것은 음성 테스트입니다. 청취하여 좋아하는 음성을 선택할 수 있습니다."
    } else if voice.starts_with("es-") {
        // 西班牙语音色
        "Hola, esto es una prueba de voz. Puedes usarla para previsualizar y elegir tu voz favorita."
    } else if voice.starts_with("fr-") {
        // 法语音色
        "Bonjour, ceci est un test vocal. Vous pouvez l'utiliser pour prévisualiser et choisir votre voix préférée."
    } else if voice.starts_with("it-") {
        // 意大利语音色
        "Ciao, questo è un test vocale. Puoi usarlo per visualizzare in anteprima e scegliere la tua voce preferita."
    } else if voice.starts_with("pt-PT") {
        // 葡萄牙语（葡萄牙）音色
        "Olá, este é um teste de voz. Pode usá-lo para pré-visualizar e escolher a sua voz favorita."
    } else if voice.starts_with("pt-BR") {
        // 葡萄牙语（巴西）音色
        "Olá, isto é um teste de voz. Você pode usá-lo para visualizar e escolher sua voz favorita."
    } else {
        // 默认英文
        "Hello, this is a voice test. You can use this to preview and choose your favorite voice."
    };

    log::info!("Generating voice preview: voice={}, use_edge={}, text={}", voice, use_edge, preview_text);

    // 优先使用 Azure TTS（如果有 API key）
    if let Some(ref key) = api_key {
        if !key.is_empty() {
            let region_str = region.unwrap_or_else(|| "eastus".to_string());
            let preview_path = std::env::temp_dir().join("preview_azure.wav");
            match call_azure_tts(preview_text, &azure_voice, 1.0, &preview_path, key, &region_str) {
                Ok(_) => {
                    let audio_data = tokio::fs::read(&preview_path).await.map_err(|e| format!("Failed to read preview audio: {}", e))?;
                    let base64_data = base64::engine::general_purpose::STANDARD.encode(&audio_data);
                    let data_url = format!("data:audio/wav;base64,{}", base64_data);
                    let _ = tokio::fs::remove_file(preview_path).await;
                    log::info!("Azure voice preview generated successfully, size: {} bytes", audio_data.len());
                    return Ok(data_url);
                }
                Err(e) => {
                    log::warn!("Azure TTS preview failed: {}, trying Edge TTS...", e);
                }
            }
        }
    }

    // 如果 use_edge 关闭且没有 API key，保底启用 Edge TTS
    let api_key_empty = api_key.as_ref().map(|k| k.is_empty()).unwrap_or(true);
    let use_edge = if !use_edge && api_key_empty { true } else { use_edge };

    // 回退到 Edge TTS（如果启用）
    if use_edge {
        let python_script = format!(r#"
import asyncio
import edge_tts
import sys

async def main():
    communicate = edge_tts.Communicate("{}", "{}")
    await communicate.save("preview_output.wav")
    print("EDGE_TTS_SUCCESS")

try:
    asyncio.run(main())
except Exception as e:
    print(f"EDGE_TTS_ERROR:{{e}}")
    sys.exit(1)
"#, preview_text, voice);

        let python_exe = get_python_exe_path()?;
        log::info!("Using Python for preview: {}", python_exe.display());

        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let output = std::process::Command::new(&python_exe)
            .args(["-c", &python_script])
            .creation_flags(CREATE_NO_WINDOW)
            .current_dir(std::env::temp_dir())
            .output()
            .map_err(|e| format!("Failed to run Edge TTS: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Edge TTS error: {}", stderr);
            return Err(format!("Voice preview failed: {}", stderr));
        }

        let preview_path = std::env::temp_dir().join("preview_output.wav");
        let audio_data = tokio::fs::read(&preview_path).await.map_err(|e| format!("Failed to read preview audio: {}", e))?;
        let base64_data = base64::engine::general_purpose::STANDARD.encode(&audio_data);
        let data_url = format!("data:audio/wav;base64,{}", base64_data);

        // 清理临时文件
        let _ = tokio::fs::remove_file(preview_path).await;

        log::info!("Edge voice preview generated successfully, size: {} bytes", audio_data.len());
        return Ok(data_url);
    }

    // 两个都不可用
    Err("Both Azure TTS and Edge TTS are disabled. Please enable at least one in Settings.".to_string())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("G-Reader starting...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            generate_speech,
            save_settings,
            get_settings_command,
            open_settings_window,
            validate_azure_api_key,
            extract_pdf_text,
            read_audio_file,
            preview_voice,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
