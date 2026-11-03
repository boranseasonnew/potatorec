# Potato Recorder — Português (Brasil)

**Guia do usuário**

Um pequeno estúdio nativo do Windows para gravação de jogos.

## Primeiros passos

Instale o FFmpeg 8.1 ou mais recente e o ffprobe correspondente. Coloque-os no PATH ou ao lado do aplicativo. O aplicativo não baixa ferramentas nem carrega gravações.

Abra Configurações para selecionar seu idioma. Em Gravação, selecione uma tela ou janela de jogo. O Windows Graphics Capture oferece suporte a janelas e monitores; DXGI oferece suporte a monitores.

## Vídeo e desempenho

Comece com 1280 × 720 a 60 FPS em gráficos integrados e depois meça os tempos de quadro do jogo. O padrão continua sendo a resolução de origem. Esta é uma recomendação, não uma garantia de desempenho.

Testa automaticamente codificadores de hardware reais. Quantização de mudanças de qualidade; a velocidade muda o esforço do codificador. Taxas de quadros fracionárias exatas são suportadas até 120 FPS. A entrega real da fonte pode ser menor.

A transferência de compatibilidade padrão baixa quadros para a RAM. Não é cópia zero. Trabalhadores extras podem aumentar a contenção. Os perfis nunca alteram as configurações do jogo ou diminuem silenciosamente a resolução ou FPS.

## Áudio

O microfone está desligado por padrão. Habilite-o explicitamente em Áudio e escolha seu dispositivo. O som da área de trabalho captura o endpoint de saída selecionado, não apenas um jogo. Use fones de ouvido.

Ative o roteamento de áudio misto e trilha independente conforme necessário. O arquivo resultante contém faixas reais separadas. RNNoise, gate, expansor, compressor, limitador e ganho podem ser habilitados e reordenados. O ruído do teclado pode permanecer.

Use o teste do microfone de oito segundos e reproduza o teste salvo para avaliar a clareza. O monitoramento ao vivo por fones de ouvido e o áudio do jogo específico do processo não são implementados.

## Controles de gravação

F9 inicia ou interrompe a gravação. F10 pausa ou retoma. F11 silencia o microfone. F7 mostra ou oculta a aplicação. F8 salva o buffer de reprodução somente de vídeo iniciado explicitamente. Atalhos conflitantes podem ser alterados em teclas de atalho.

Arraste a barra de título personalizada para mover a janela. Clique duas vezes para maximizar. Fechar durante uma gravação interrompe e finaliza com segurança. A ilustração da fonte não é uma visualização ao vivo.

## Arquivos e recuperação

MKV é o padrão. MP4 e MP4 fragmentado são finalizados após a parada. O áudio é capturado em sidecars PCM e adicionado após a parada. Sessões longas podem levar algum tempo para serem finalizadas.

Cada fonte de áudio estéreo habilitada utiliza cerca de 1,38 GB por hora em PCM temporário, além de vídeo e cópias intermediárias. As pastas de recuperação mantêm os originais. Verifique a gravação concluída antes de remover manualmente a pasta da sessão.

Use Gravações para abrir arquivos, remuxar MKV para MP4 sem recodificação de vídeo ou recuperar sessões inacabadas. As gravações originais não são excluídas automaticamente.

## Limites e verificação

Atualmente, o Replay salva apenas vídeo. HDR, otimização adaptativa, isolamento de áudio de processo, monitoramento ao vivo e recuperação automática de redefinição de GPU estão inacabados. O alinhamento de áudio/vídeo de inicialização é estimado; verifique a sincronização labial antes de gravações importantes.

Conteúdo protegido, alguns jogos exclusivos em tela cheia e janelas minimizadas podem parar de entregar frames. Alt+Tab mantém a sessão aberta, mas não pode remover restrições do sistema operacional.

Nenhuma vantagem de desempenho em relação ao OBS ou Xbox Game Bar foi estabelecida. Consulte o protocolo de benchmark e o status de implementação para testes reproduzíveis e trabalho restante.

## Nota de tradução

Este pacote de idiomas foi assistido por máquina e ainda não recebeu uma revisão completa por falantes nativos. Os diagnósticos técnicos do Windows e FFmpeg mantêm seu texto original.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
