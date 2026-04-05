!macro NSIS_HOOK_POSTINSTALL
  ; 解压 Python 运行时环境到安装目录
  ; Python.zip 由 Tauri resources 打包，放在安装目录根下
  nsisunz::UnzipToLog "$INSTDIR\python.zip" "$INSTDIR"
  Pop $0
  ${If} $0 != "success"
    DetailPrint "Python 解压失败: $0"
  ${Else}
    DetailPrint "Python 运行时解压完成"
    Delete "$INSTDIR\python.zip"
  ${EndIf}
!macroend
