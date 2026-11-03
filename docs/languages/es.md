# Potato Recorder — Español

**guía de usuario**

Un pequeño estudio nativo de Windows para grabar juegos.

## Empezando

Instale FFmpeg 8.1 o posterior y el ffprobe correspondiente. Póngalos en PATH o al lado de la aplicación. La aplicación no descarga herramientas ni carga grabaciones.

Abra Configuración para seleccionar su idioma. En Grabación, seleccione una pantalla o ventana de juego. Windows Graphics Capture admite ventanas y pantallas; DXGI admite pantallas.

## Vídeo y actuación

Comience con 1280 × 720 a 60 FPS en gráficos integrados y luego mida los tiempos de cuadros del juego. El valor predeterminado sigue siendo la resolución de origen. Esta es una recomendación, no una garantía de rendimiento.

Pruebas automáticas de codificadores de hardware reales. Cuantización de cambios de calidad; La velocidad cambia el esfuerzo del codificador. Se admiten velocidades de fotogramas fraccionales exactas hasta 120 FPS. La entrega de origen real puede ser menor.

La transferencia de compatibilidad predeterminada descarga fotogramas a la RAM. No es copia cero. Los trabajadores adicionales pueden aumentar la contención. Los perfiles nunca cambian la configuración del juego ni reducen silenciosamente la resolución o FPS.

## Audio

El micrófono está apagado de forma predeterminada. Habilítelo explícitamente en Audio y elija su dispositivo. El sonido del escritorio captura el punto final de salida seleccionado, no solo un juego. Utilice auriculares.

Habilite el enrutamiento de audio mixto y pista independiente según sea necesario. El archivo resultante contiene pistas reales separadas. RNNoise, puerta, expansor, compresor, limitador y ganancia se pueden habilitar y reordenar. Es posible que persista el ruido del teclado.

Utilice la prueba del micrófono de ocho segundos y reproduzca la prueba guardada para juzgar la claridad. No se implementan el monitoreo de auriculares en vivo ni el audio del juego específico del proceso.

## Controles de grabación

F9 inicia o detiene la grabación. F10 pausa o reanuda. F11 silencia el micrófono. F7 muestra u oculta la aplicación. F8 guarda el búfer de reproducción de solo video iniciado explícitamente. Los atajos conflictivos se pueden cambiar en las teclas de acceso rápido.

Arrastre la barra de título personalizada para mover la ventana. Haga doble clic en él para maximizar. El cierre durante una grabación la detiene y finaliza de forma segura. La ilustración original no es una vista previa en vivo.

## Archivos y recuperación

MKV es el predeterminado. MP4 y MP4 fragmentado se finalizan después de detenerse. El audio se captura en sidecars PCM y se agrega después de la parada. Las sesiones largas pueden tardar en finalizar.

Cada fuente de audio estéreo habilitada utiliza aproximadamente 1,38 GB por hora en PCM temporal, además de vídeo y copias intermedias. Las carpetas de recuperación conservan los originales. Verifique la grabación completada antes de eliminar manualmente su carpeta de sesión.

Utilice Grabaciones para abrir archivos, remuxar MKV a MP4 sin volver a codificar el vídeo o recuperar sesiones inacabadas. Las grabaciones originales no se eliminan automáticamente.

## Límites y verificación

Actualmente, la reproducción solo guarda el vídeo. HDR, optimización adaptativa, aislamiento de audio de proceso, monitoreo en vivo y recuperación automática de reinicio de GPU están inconclusos. Se estima la alineación de audio/vídeo de inicio; verifique la sincronización de labios antes de grabaciones importantes.

El contenido protegido, algunos juegos exclusivos en pantalla completa y ventanas minimizadas pueden dejar de mostrar fotogramas. Alt+Tab mantiene la sesión abierta, pero no puede eliminar las restricciones del sistema operativo.

No se ha establecido ninguna ventaja de rendimiento sobre OBS o Xbox Game Bar. Consulte el protocolo de referencia y el estado de implementación para conocer las pruebas reproducibles y el trabajo restante.

## Nota de traducción

Este paquete de idioma fue asistido por máquina y aún no ha recibido una revisión completa por parte de un hablante nativo. Los diagnósticos técnicos de Windows y FFmpeg conservan su redacción original.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
