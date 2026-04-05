# MINI TTS 语音合成工具 — 中文使用指南

![MINI TTS](icons/128x128.png)

## 功能特点

- **双引擎支持**：Azure TTS（高品质）+ Edge TTS（免费备用）
- **零依赖运行**：内置 Python 和 edge-tts，解压即用
- **多语言支持**：中文、英文、日文、韩文、法语、德语等 20+ 种语言
- **丰富音色**：每个语种提供多种音色可选（男声/女声）
- **语速调节**：0.5x ~ 2.0x 自由调节
- **自动检测**：自动识别系统语言，匹配对应音色
- **暗色主题**：现代化深色界面，保护眼睛

---

## 系统要求

- Windows 10/11（64 位）
- 无需安装 Python
- 无需安装任何运行时

---

## 安装步骤

1. 双击运行 `Mini TTS_1.0.0_x64-setup.exe`
2. 按向导提示完成安装
3. 从桌面快捷方式启动程序

---

## 界面介绍

```
┌─────────────────────────────────────────────────────────┐
│  ✕ 标题栏：拖动移动 | 最小化 | 关闭                      │
├─────────────────────────────────────────────────────────┤
│  Language: [中文 ▼]   Voice: [晓晓 ▼]   📂  💾          │
├─────────────────────────────────────────────────────────┤
│  Rate: ━━━●━━━ 1.0x  [Apply]   PAUSE: [0.5s] [1s]       │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  输入要转换的文字...                               │   │
│  │                                                  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  🔊 ━○━━▶─────────── ⏹ Open Audio  [  EXECUTE  ]      │
└─────────────────────────────────────────────────────────┘
```

---

## 基本使用方法

### 第一步：输入文字
在中央文本框输入或粘贴要转语音的文字（支持中英文混排）。

### 第二步：选择语言
点击 **Language** 下拉菜单，选择文字对应的语言。
> 程序会自动识别系统语言，首次启动默认选中对应语言。

### 第三步：选择音色
点击 **Voice** 下拉菜单，选择喜欢的音色。
> 每个语言提供多种音色，涵盖男声、女声及不同风格。

### 第四步：调节语速（可选）
拖动 **Rate** 滑块，范围 0.5x ~ 2.0x，点击 **Apply** 确认。
> 1.0x 为正常语速。

### 第五步：生成语音
点击右下角 **EXECUTE** 按钮，稍等片刻。
> 首次使用 Edge TTS 约需 3-5 秒生成；Azure TTS 约需 1-3 秒。

### 第六步：播放或打开音频
- 点击绿色 ▶ 按钮播放生成的音频
- 点击 **Open Audio** 打开音频文件所在文件夹

---

## 设置保存路径

点击 **Save To** 按钮，可以指定音频文件的保存位置（默认保存到用户下载文件夹）。

---

## Azure TTS 配置（可选）

Azure TTS 提供更高品质的语音，适合专业场景。

> **免费畅享世界顶级音质** — Azure 语音服务为新用户提供**免费层额度**，每月可免费合成 50 万字符，足够个人日常使用。我们正在逐步接入更多优质 API 服务商，让每一位用户都能零成本体验前沿语音技术。

### 获取 Azure API Key
1. 访问 [Azure 语音服务](https://azure.microsoft.com/zh-cn/services/cognitive-services/speech-services/)
2. 点击"免费开始使用"，用邮箱或 Microsoft 账户注册
3. 完成身份验证后，在 Azure 门户创建"语音服务"资源
4. 选择**免费层（F0）**，点击创建
5. 进入资源后，在"密钥和终结点"中复制 **Key 1** 和 **Region（区域）**
6. 将 Key 和 Region 填入程序设置，保存即可

> **免费层说明**：Azure 免费层每月包含 50 万字符的语音配额，个人学习、创作、娱乐使用完全足够。超出部分按量计费，默认不会自动扣费。

### 配置程序
1. 点击右上角 ⚙ 设置图标
2. 粘贴 API Key
3. 选择对应的 Region（默认 eastus）
4. 点击 **Save** 保存
5. 重启程序使设置生效

> **免费额度**：每月 50 万字符，足够个人日常使用。
- 已配置 Azure API Key → 优先使用 Azure TTS
- 未配置或 Azure 调用失败 → 自动切换到 Edge TTS（免费）

---

## 语音引擎对比

| 引擎 | 音质 | 语种覆盖 | 免费额度 | 配置要求 |
|------|------|---------|---------|---------|
| Azure TTS | ⭐⭐⭐⭐⭐ 专业级 | 120+ 种 | 每月 50 万字符免费 | API Key |
| Edge TTS | ⭐⭐⭐⭐ 良好 | 80+ 种 | 完全免费，无限制 | 无，开箱即用 |

---

## 隐私说明

- **本地存储**：API Key 仅保存在本地配置文件，不会上传
- **文本处理**：文字转语音通过 Azure 或 Edge 官方服务器处理
- **匿名统计**：程序收集匿名的使用统计（首次启动弹窗说明），用于改进产品

---

## 常见问题

**Q: 点击 EXECUTE 后没有反应？**
> 检查文本框是否为空，确保已输入文字。

**Q: 生成失败，提示错误？**
> 请检查网络连接。Edge TTS 依赖网络调用 Edge 服务器。

**Q: 如何选择中文方言？**
> Language 选择 Chinese (Mandarin) 后，Voice 中包含"小贝"（辽宁话）、"小妮"（陕西话）等方言选项。

**Q: 可以调整语音停顿吗？**
> 可以。在 Rate 控制栏旁边有 PAUSE 按钮，点击可插入 0.5s 或 1s 的停顿。

**Q: 如何清除已设置的保存路径？**
> 关闭程序后删除配置文件夹 `C:\Users\你的用户名\AppData\Roaming\com.greader.app` 下的设置文件，重启程序即可重置。

---

## 技术信息

- 版本：1.0.0
- 开发框架：Tauri + Rust
- 内置 Python：3.10
- 语音引擎：edge-tts 7.2.7

---

## 卸载

1. 运行安装目录下 `uninstall.exe`
2. 或在 Windows 设置 → 应用 → 已安装应用 中找到 **Mini TTS** 卸载

---

联系邮箱：bimongl@gmail.com

---

# MINI TTS — English User Guide

![MINI TTS](icons/128x128.png)

## Features

- **Dual Engine**: Azure TTS (high quality) + Edge TTS (free backup)
- **Zero Dependencies**: Built-in Python and edge-tts, no installation required
- **Multi-language**: 20+ languages including Chinese, English, Japanese, Korean, French, German, and more
- **Voice Variety**: Multiple voices per language (male/female)
- **Speed Control**: Adjustable from 0.5x to 2.0x
- **Auto Language Detection**: Automatically detects your system language and selects the matching voice
- **Dark Theme**: Modern dark UI, easy on the eyes

---

## System Requirements

- Windows 10/11 (64-bit)
- No Python installation needed
- No additional runtime required

---

## Installation

1. Double-click `Mini TTS_1.0.0_x64-setup.exe`
2. Follow the setup wizard
3. Launch from the desktop shortcut

---

## Interface Overview

```
┌─────────────────────────────────────────────────────────┐
│  ✕  Title Bar: drag to move | Minimize | Close          │
├─────────────────────────────────────────────────────────┤
│  Language: [English ▼]   Voice: [Jenny ▼]   📂  💾       │
├─────────────────────────────────────────────────────────┤
│  Rate: ━━━●━━━ 1.0x  [Apply]   PAUSE: [0.5s] [1s]       │
│                                                         │
│  ┌─────────────────────────────────────────────────┐    │
│  │  Enter text to synthesize...                     │    │
│  │                                                  │    │
│  └─────────────────────────────────────────────────┘    │
│                                                         │
│  🔊 ━○━━▶─────────── ⏹ Open Audio  [  EXECUTE  ]       │
└─────────────────────────────────────────────────────────┘
```

---

## How to Use

### Step 1 — Enter Text
Type or paste the text you want to convert to speech in the main text area. Mixed Chinese and English text is fully supported.

### Step 2 — Select Language
Click the **Language** dropdown and choose the language of your text.
> On first launch, the program automatically detects your system language and pre-selects it.

### Step 3 — Select Voice
Click the **Voice** dropdown and choose your preferred voice.
> Each language has multiple voice options, including male, female, and various styles.

### Step 4 — Adjust Speed (Optional)
Drag the **Rate** slider, range 0.5x ~ 2.0x, click **Apply** to confirm.
> 1.0x is normal speed.

### Step 5 — Generate Speech
Click the **EXECUTE** button in the bottom-right corner and wait a moment.
> First-time generation takes about 3-5 seconds with Edge TTS; Azure TTS takes about 1-3 seconds.

### Step 6 — Play or Open Audio
- Click the green ▶ button to play the generated audio
- Click **Open Audio** to open the folder where the audio file is saved

---

## Setting a Save Location

Click the **Save To** button to choose where audio files are saved. By default, files are saved to your Downloads folder.

---

## Azure TTS Setup (Optional)

Azure TTS provides higher quality voices, ideal for professional use.

> **Premium Quality, At No Cost** — Azure Speech Services offers a **free tier** for all new users, providing 500,000 free characters per month — more than enough for personal daily use. We're actively integrating more leading API providers into this app, so every user can enjoy cutting-edge voice technology at zero cost.

### Get Your Azure API Key
1. Visit [Azure Speech Services](https://azure.microsoft.com/services/cognitive-services/speech-services/)
2. Click **"Get started for free"** and sign up with your email or Microsoft account
3. Complete identity verification, then create a **"Speech Services"** resource in the Azure portal
4. Select the **Free (F0)** pricing tier, then click **Create**
5. Once deployed, go to **"Keys and Endpoint"** and copy **Key 1** and your **Region**
6. Paste both into the app settings and click **Save** — you're all set

> **Free Tier Details**: Azure's free tier includes 500,000 characters of speech synthesis per month — perfectly sufficient for learning, creative projects, and everyday use. Usage beyond the free quota is pay-as-you-go and will not be charged without your explicit consent.

### Configure the App
1. Click the ⚙ settings icon in the top-right corner
2. Paste your API Key
3. Select the matching Region (default: eastus)
4. Click **Save**
5. Restart the app for changes to take effect

> **Free Quota**: 500,000 characters per month, sufficient for daily personal use.
- Azure API Key configured → Azure TTS is used first
- Not configured or Azure fails → Automatically falls back to Edge TTS (free)

---

## Engine Comparison

| Engine | Quality | Languages | Free Quota | Setup Required |
|--------|---------|-----------|-----------|----------------|
| Azure TTS | ⭐⭐⭐⭐⭐ Professional | 120+ | 500K chars/month free | API Key |
| Edge TTS | ⭐⭐⭐⭐ Good | 80+ | Fully free, unlimited | None, out of the box |

---

## Privacy Notice

- **Local Storage**: API Key is stored only in your local config file, never uploaded
- **Text Processing**: Text-to-speech is handled by Azure or Edge official servers
- **Anonymous Statistics**: The app collects anonymous usage statistics (explained on first launch), used to improve the product

---

## FAQ

**Q: Nothing happens when I click EXECUTE?**
> Make sure the text area is not empty — you must enter some text first.

**Q: Generation failed with an error?**
> Please check your internet connection. Edge TTS requires network access to call Edge servers.

**Q: How do I select a Chinese dialect?**
> After selecting Chinese (Mandarin) as the Language, the Voice dropdown includes regional accents like "Xiaobei" (Liaoning) and "Xiaoni" (Shaanxi).

**Q: Can I add speech pauses?**
> Yes. Next to the Rate control, the PAUSE buttons insert 0.5s or 1s pauses into the speech output.

**Q: How do I reset the save path?**
> Close the app and delete the settings file in `C:\Users\YourUsername\AppData\Roaming\com.greader.app`, then restart the app.

---

## Technical Info

- Version: 1.0.0
- Framework: Tauri + Rust
- Built-in Python: 3.10
- Voice Engine: edge-tts 7.2.7

---

## Uninstall

1. Run `uninstall.exe` in the installation folder
2. Or go to Windows Settings → Apps → Installed apps → **Mini TTS** → Uninstall

---

Contact: bimongl@gmail.com
