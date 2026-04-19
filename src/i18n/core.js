// i18n Core - Internationalization module for Mini TTS
// Translations are embedded to avoid network latency

const i18n = {
  currentLocale: 'en',
  translations: {},
  initialized: false,

  // Font mapping for each language (slim-tall fonts for CJK)
  fontMap: {
    'zh': '"Microsoft JhengHei Light", "Microsoft JhengHei", sans-serif',
    'ja': '"Yu Gothic", "Meiryo", sans-serif',
    'ko': '"Malgun Gothic SemiLight", "Malgun Gothic", sans-serif',
    'en': '"JetBrains Mono", Consolas, "Courier New", monospace',
    'de': '"JetBrains Mono", Consolas, "Courier New", monospace',
    'fr': '"JetBrains Mono", Consolas, "Courier New", monospace',
    'es': '"JetBrains Mono", Consolas, "Courier New", monospace',
    'default': 'system-ui, -apple-system, sans-serif'
  },

  // Language code to locale file mapping
  localeMap: {
    'zh': 'zh',
    'ja': 'ja',
    'ko': 'ko',
    'en': 'en',
    'de': 'de',
    'fr': 'fr',
    'es': 'es'
  },

  // Embedded translations
  locales: {
    en: {
      "app_title": "Mini TTS", "language": "Language", "ui_language": "UI Language", "voice": "Voice",
      "open": "Open", "save_to": "Save To", "not_set": "Not set", "input_buffer": "Input Buffer",
      "rate": "Rate", "apply": "Apply", "pause": "PAUSE", "chars": "chars", "execute": "Execute",
      "open_audio": "Open Audio", "minimize": "Minimize", "maximize": "Maximize", "close": "Close",
      "privacy_notice": "Privacy Notice", "privacy_p1": "Mini TTS collects anonymous usage data to improve the product. This includes:",
      "privacy_anon_id": "Anonymous device ID (randomly generated)", "privacy_stats": "Usage statistics (launch count, button clicks, session duration)",
      "privacy_not_collect": "We do NOT collect:", "privacy_no_personal": "Personal information",
      "privacy_no_text": "Input text content", "privacy_no_recordings": "Voice recordings",
      "privacy_consent": "By clicking \"I Agree\", you consent to the collection of anonymous usage data.",
      "disagree": "Disagree", "i_agree": "I Agree", "preview": "Preview", "set_path_first": "SET PATH FIRST",
      "synthesizing": "SYNTHESIZING...", "success": "SUCCESS", "failed": "FAILED", "ready": "READY",
      "playing": "PLAYING", "paused": "PAUSED", "stopped": "STOPPED", "no_audio": "No audio loaded",
      "load_error": "Failed to load audio", "settings": "Settings", "api_key": "API Key", "region": "Region",
      "enable_azure": "Enable Azure TTS", "enable_edge": "Enable Edge TTS", "save": "Save", "cancel": "Cancel",
      "api_key_valid": "API Key is valid", "api_key_invalid": "API Key is invalid", "validating": "Validating...",
      "bg_music": "Background Music", "bg_volume": "Volume", "mix_complete": "Mix complete", "mix_failed": "Mix failed",
      "mixing": "MIXING...", "error": "ERROR", "azure_tts_config": "Azure TTS Configuration", "enable": "Enable",
      "edge_tts_config": "Fallback: Edge TTS", "edge_tts_desc": "Use Edge TTS when Azure fails (free)"
    },
    zh: {
      "app_title": "Mini TTS", "language": "语言", "ui_language": "界面语言", "voice": "语音",
      "open": "打开", "save_to": "保存到", "not_set": "未设置", "input_buffer": "输入缓冲区",
      "rate": "语速", "apply": "应用", "pause": "停顿", "chars": "字符", "execute": "执行",
      "open_audio": "打开音频", "minimize": "最小化", "maximize": "最大化", "close": "关闭",
      "privacy_notice": "隐私声明", "privacy_p1": "Mini TTS 收集匿名使用数据以改进产品，包括：",
      "privacy_anon_id": "匿名设备ID（随机生成）", "privacy_stats": "使用统计（启动次数、按钮点击、会话时长）",
      "privacy_not_collect": "我们不收集：", "privacy_no_personal": "个人信息",
      "privacy_no_text": "输入文本内容", "privacy_no_recordings": "语音录音",
      "privacy_consent": "点击「同意」，即表示您同意收集匿名使用数据。",
      "disagree": "不同意", "i_agree": "同意", "preview": "预览", "set_path_first": "请先设置路径",
      "synthesizing": "合成中...", "success": "成功", "failed": "失败", "ready": "就绪",
      "playing": "播放中", "paused": "已暂停", "stopped": "已停止", "no_audio": "未加载音频",
      "load_error": "加载音频失败", "settings": "设置", "api_key": "API密钥", "region": "区域",
      "enable_azure": "启用 Azure TTS", "enable_edge": "启用 Edge TTS", "save": "保存", "cancel": "取消",
      "api_key_valid": "API密钥有效", "api_key_invalid": "API密钥无效", "validating": "验证中...",
      "bg_music": "背景音乐", "bg_volume": "音量", "mix_complete": "混合完成", "mix_failed": "混合失败",
      "mixing": "混合中...", "error": "错误", "azure_tts_config": "Azure TTS 配置", "enable": "启用",
      "edge_tts_config": "备用：Edge TTS", "edge_tts_desc": "当 Azure 失败时使用 Edge TTS（免费）"
    },
    ja: {
      "app_title": "Mini TTS", "language": "言語", "ui_language": "表示言語", "voice": "音声",
      "open": "開く", "save_to": "保存先", "not_set": "未設定", "input_buffer": "入力バッファ",
      "rate": "速度", "apply": "適用", "pause": "一時停止", "chars": "文字", "execute": "実行",
      "open_audio": "オーディオを開く", "minimize": "最小化", "maximize": "最大化", "close": "閉じる",
      "privacy_notice": "プライバシー通知", "privacy_p1": "Mini TTS は製品改善のために匿名の使用データを収集します。内容包括：",
      "privacy_anon_id": "匿名のデバイスID（ランダム生成）", "privacy_stats": "使用統計（起動回数、ボタンクリック、セッション時間）",
      "privacy_not_collect": "私たちは収集しません：", "privacy_no_personal": "個人情報",
      "privacy_no_text": "入力テキスト内容", "privacy_no_recordings": "音声録音",
      "privacy_consent": "「同意」をクリックすると、匿名の使用データの収集に同意したものとみなされます。",
      "disagree": "不同意", "i_agree": "同意", "preview": "プレビュー", "set_path_first": "パスを設定してください",
      "synthesizing": "合成中...", "success": "成功", "failed": "失敗", "ready": "準備完了",
      "playing": "再生中", "paused": "一時停止中", "stopped": "停止済み", "no_audio": "オーディオ未装载",
      "load_error": "オーディオの読み込みに失敗しました", "settings": "設定", "api_key": "APIキー", "region": "地域",
      "enable_azure": "Azure TTS を有効にする", "enable_edge": "Edge TTS を有効にする", "save": "保存", "cancel": "キャンセル",
      "api_key_valid": "APIキーは有効です", "api_key_invalid": "APIキーは無効です", "validating": "検証中...",
      "bg_music": "BGM", "bg_volume": "音量", "mix_complete": "ミキシング完了", "mix_failed": "ミキシング失敗",
      "mixing": "ミキシング中...", "error": "エラー", "azure_tts_config": "Azure TTS 設定", "enable": "有効",
      "edge_tts_config": "予備：Edge TTS", "edge_tts_desc": "Azure失敗時にEdge TTSを使用（無料）"
    },
    ko: {
      "app_title": "Mini TTS", "language": "언어", "ui_language": "인터페이스 언어", "voice": "음성",
      "open": "열기", "save_to": "저장 위치", "not_set": "설정되지 않음", "input_buffer": "입력 버퍼",
      "rate": "속도", "apply": "적용", "pause": "일시정지", "chars": "자", "execute": "실행",
      "open_audio": "오디오 열기", "minimize": "최소화", "maximize": "최대화", "close": "닫기",
      "privacy_notice": "개인정보 보호 고지", "privacy_p1": "Mini TTS는 제품 개선을 위해 익명의 사용 데이터를 수집합니다. 포함되는 내용：",
      "privacy_anon_id": "익명 기기 ID（무작위 생성）", "privacy_stats": "사용 통계（실행 횟수, 버튼 클릭, 세션 시간）",
      "privacy_not_collect": "우리는 수집하지 않습니다：", "privacy_no_personal": "개인 정보",
      "privacy_no_text": "입력 텍스트 내용", "privacy_no_recordings": "음성 녹음",
      "privacy_consent": "\"동의\"를 클릭하면 익명의 사용 데이터 수집에 동의하는 것입니다.",
      "disagree": "동의 안 함", "i_agree": "동의", "preview": "미리보기", "set_path_first": "경로를 설정하세요",
      "synthesizing": "합성 중...", "success": "성공", "failed": "실패", "ready": "준비",
      "playing": "재생 중", "paused": "일시 정지됨", "stopped": "정지됨", "no_audio": "오디오가 로드되지 않음",
      "load_error": "오디오 로드 실패", "settings": "설정", "api_key": "API 키", "region": "지역",
      "enable_azure": "Azure TTS 활성화", "enable_edge": "Edge TTS 활성화", "save": "저장", "cancel": "취소",
      "api_key_valid": "API 키가 유효합니다", "api_key_invalid": "API 키가 유효하지 않습니다", "validating": "유효성 검사 중...",
      "bg_music": "배경 음악", "bg_volume": "볼륨", "mix_complete": "믹싱 완료", "mix_failed": "믹싱 실패",
      "mixing": "믹싱 중...", "error": "오류", "azure_tts_config": "Azure TTS 설정", "enable": "활성화",
      "edge_tts_config": "백업：Edge TTS", "edge_tts_desc": "Azure 실패 시 Edge TTS 사용（무료）"
    },
    de: {
      "app_title": "Mini TTS", "language": "Sprache", "voice": "Stimme", "open": "Öffnen", "save_to": "Speichern unter",
      "not_set": "Nicht festgelegt", "input_buffer": "Eingabepuffer", "rate": "Geschwindigkeit", "apply": "Anwenden",
      "pause": "Pause", "chars": "Zeichen", "execute": "Ausführen", "open_audio": "Audio öffnen",
      "minimize": "Minimieren", "maximize": "Maximieren", "close": "Schließen", "privacy_notice": "Datenschutzhinweis",
      "privacy_p1": "Mini TTS erfasst anonyme Nutzungsdaten zur Produktverbesserung. Dies beinhaltet:",
      "privacy_anon_id": "Anonyme Geräte-ID (zufällig generiert)", "privacy_stats": "Nutzungsstatistiken (Startanzahl, Button-Klicks, Sitzungsdauer)",
      "privacy_not_collect": "Wir erfassen NICHT:", "privacy_no_personal": "Persönliche Informationen",
      "privacy_no_text": "Eingetippte Textinhalte", "privacy_no_recordings": "Sprachaufnahmen",
      "privacy_consent": "Mit einem Klick auf «Zustimmen» stimmen Sie der Erfassung anonymer Nutzungsdaten zu.",
      "disagree": "Ablehnen", "i_agree": "Zustimmen", "preview": "Vorschau", "set_path_first": "Pfad zuerst festlegen",
      "synthesizing": "SYNTHESE LÄUFT...", "success": "ERFOLG", "failed": "FEHLGESCHLAGEN", "ready": "BEREIT",
      "playing": "WIEDERGABE", "paused": "PAUSE", "stopped": "GESTOPPT", "no_audio": "Kein Audio geladen",
      "load_error": "Audio konnte nicht geladen werden", "settings": "Einstellungen", "api_key": "API-Schlüssel", "region": "Region",
      "enable_azure": "Azure TTS aktivieren", "enable_edge": "Edge TTS aktivieren", "save": "Speichern", "cancel": "Abbrechen",
      "api_key_valid": "API-Schlüssel ist gültig", "api_key_invalid": "API-Schlüssel ist ungültig", "validating": "Validierung läuft...",
      "bg_music": "Hintergrundmusik", "bg_volume": "Lautstärke", "mix_complete": "Mischung abgeschlossen", "mix_failed": "Mischung fehlgeschlagen",
      "mixing": "MISCHUNG LÄUFT...", "error": "FEHLER", "azure_tts_config": "Azure TTS Konfiguration", "enable": "Aktivieren",
      "edge_tts_config": "Fallback: Edge TTS", "edge_tts_desc": "Edge TTS verwenden, wenn Azure fehlschlägt (kostenlos)"
    },
    fr: {
      "app_title": "Mini TTS", "language": "Langue", "voice": "Voix", "open": "Ouvrir", "save_to": "Enregistrer sous",
      "not_set": "Non défini", "input_buffer": "Tampon d'entrée", "rate": "Vitesse", "apply": "Appliquer",
      "pause": "Pause", "chars": "caractères", "execute": "Exécuter", "open_audio": "Ouvrir l'audio",
      "minimize": "Réduire", "maximize": "Agrandir", "close": "Fermer", "privacy_notice": "Avis de confidentialité",
      "privacy_p1": "Mini TTS collecte des données d'utilisation anonymes pour améliorer le produit. Cela comprend :",
      "privacy_anon_id": "ID d'appareil anonyme (généré aléatoirement)", "privacy_stats": "Statistiques d'utilisation (nombre de lancements, clics, durée de session)",
      "privacy_not_collect": "Nous ne collectons PAS :", "privacy_no_personal": "Informations personnelles",
      "privacy_no_text": "Contenu du texte saisi", "privacy_no_recordings": "Enregistrements vocaux",
      "privacy_consent": "En cliquant sur « J'accepte », vous consentez à la collecte de données d'utilisation anonymes.",
      "disagree": "Refuser", "i_agree": "J'accepte", "preview": "Aperçu", "set_path_first": "Définir le chemin d'abord",
      "synthesizing": "SYNTHÈSE EN COURS...", "success": "SUCCÈS", "failed": "ÉCHEC", "ready": "PRÊT",
      "playing": "LECTURE", "paused": "EN PAUSE", "stopped": "ARRÊTÉ", "no_audio": "Aucun audio chargé",
      "load_error": "Échec du chargement de l'audio", "settings": "Paramètres", "api_key": "Clé API", "region": "Région",
      "enable_azure": "Activer Azure TTS", "enable_edge": "Activer Edge TTS", "save": "Enregistrer", "cancel": "Annuler",
      "api_key_valid": "La clé API est valide", "api_key_invalid": "La clé API est invalide", "validating": "Validation en cours...",
      "bg_music": "Musique de fond", "bg_volume": "Volume", "mix_complete": "Mixage terminé", "mix_failed": "Échec du mixage",
      "mixing": "MIXAGE EN COURS...", "error": "ERREUR", "azure_tts_config": "Configuration Azure TTS", "enable": "Activer",
      "edge_tts_config": "Secours : Edge TTS", "edge_tts_desc": "Utiliser Edge TTS en cas d'échec d'Azure (gratuit)"
    },
    es: {
      "app_title": "Mini TTS", "language": "Idioma", "voice": "Voz", "open": "Abrir", "save_to": "Guardar como",
      "not_set": "No establecido", "input_buffer": "Búfer de entrada", "rate": "Velocidad", "apply": "Aplicar",
      "pause": "Pausa", "chars": "caracteres", "execute": "Ejecutar", "open_audio": "Abrir audio",
      "minimize": "Minimizar", "maximize": "Maximizar", "close": "Cerrar", "privacy_notice": "Aviso de privacidad",
      "privacy_p1": "Mini TTS recopila datos de uso anónimos para mejorar el producto. Esto incluye:",
      "privacy_anon_id": "ID de dispositivo anónimo (generado aleatoriamente)", "privacy_stats": "Estadísticas de uso (número de lanzamientos, clics, duración de sesión)",
      "privacy_not_collect": "NO recopilamos:", "privacy_no_personal": "Información personal",
      "privacy_no_text": "Contenido del texto de entrada", "privacy_no_recordings": "Grabaciones de voz",
      "privacy_consent": "Al hacer clic en \"Acepto\", aceptas la recopilación de datos de uso anónimos.",
      "disagree": "No acepto", "i_agree": "Acepto", "preview": "Vista previa", "set_path_first": "Establecer ruta primero",
      "synthesizing": "SINTETIZANDO...", "success": "ÉXITO", "failed": "FALLO", "ready": "LISTO",
      "playing": "REPRODUCIENDO", "paused": "EN PAUSA", "stopped": "DETENIDO", "no_audio": "No hay audio cargado",
      "load_error": "Error al cargar el audio", "settings": "Configuración", "api_key": "Clave API", "region": "Región",
      "enable_azure": "Habilitar Azure TTS", "enable_edge": "Habilitar Edge TTS", "save": "Guardar", "cancel": "Cancelar",
      "api_key_valid": "La clave API es válida", "api_key_invalid": "La clave API no es válida", "validating": "Validando...",
      "bg_music": "Música de fondo", "bg_volume": "Volumen", "mix_complete": "Mezcla completada", "mix_failed": "Error en la mezcla",
      "mixing": "MEZCLANDO...", "error": "ERROR", "azure_tts_config": "Configuración de Azure TTS", "enable": "Habilitar",
      "edge_tts_config": "Respaldo: Edge TTS", "edge_tts_desc": "Usar Edge TTS cuando Azure falla (gratis)"
    }
  },

  init() {
    // Prevent double initialization
    if (this.initialized) {
      return;
    }

    // Make i18n globally accessible first
    if (typeof window !== 'undefined') {
      window.i18n = this;
    }

    // Detect system language (PRIMARY, always used on first load)
    const sysLang = navigator.language || 'en';
    const langCode = sysLang.split('-')[0];
    let locale = this.localeMap[langCode] || 'en';

    // Use embedded translations - no network needed
    this.translations = this.locales[locale] || this.locales['en'];
    this.currentLocale = locale;
    this.initialized = true;
    this.applyTranslations();
    this.applyFont();
  },

  t(key) {
    return this.translations[key] || key;
  },

  applyTranslations() {
    // 合并3次 querySelectorAll 为1次，提升性能
    document.querySelectorAll('[data-i18n], [data-i18n-title], [data-i18n-placeholder]').forEach(el => {
      if (el.hasAttribute('data-i18n')) {
        el.textContent = this.t(el.getAttribute('data-i18n'));
      }
      if (el.hasAttribute('data-i18n-title')) {
        el.title = this.t(el.getAttribute('data-i18n-title'));
      }
      if (el.hasAttribute('data-i18n-placeholder')) {
        el.placeholder = this.t(el.getAttribute('data-i18n-placeholder'));
      }
    });
  },

  applyFont() {
    // 只设置 body 字体，子元素通过 inherit 继承
    const font = this.fontMap[this.currentLocale] || this.fontMap['default'];
    document.body.style.fontFamily = font;
  },

  setLocale(locale) {
    if (!this.localeMap[locale]) {
      console.warn(`Locale ${locale} not supported`);
      return;
    }
    // 如果语言没变，跳过翻译
    if (this.currentLocale === locale) {
      return;
    }
    this.translations = this.locales[locale] || this.locales['en'];
    this.currentLocale = locale;
    this.initialized = true;
    this.applyTranslations();
    this.applyFont();
    localStorage.setItem('mini-tts-locale', locale);
  }
};

// Make i18n globally accessible immediately when script loads
if (typeof window !== 'undefined') {
  window.i18n = i18n;
}
