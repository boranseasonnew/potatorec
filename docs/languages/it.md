# Potato Recorder — Italiano

**Guida per l'utente**

Un piccolo studio nativo di Windows per la registrazione del gameplay.

## Iniziare

Installa FFmpeg 8.1 o versione successiva e il ffprobe corrispondente. Inseriscili nel PERCORSO o accanto all'applicazione. L'app non scarica strumenti né carica registrazioni.

Apri Impostazioni per selezionare la lingua. In Registrazione, seleziona uno schermo o una finestra di gioco. Acquisizione grafica di Windows supporta finestre e display; DXGI supporta i display.

## Video e spettacolo

Inizia con 1280 × 720 a 60 FPS con grafica integrata, quindi misura i tempi di frame del gioco. L'impostazione predefinita rimane la risoluzione della sorgente. Questa è una raccomandazione, non una garanzia di prestazione.

Testa automaticamente gli encoder hardware reali. La qualità cambia la quantizzazione; la velocità cambia lo sforzo dell'encoder. I frame rate frazionari esatti sono supportati fino a 120 FPS. La consegna effettiva della fonte potrebbe essere inferiore.

Il trasferimento di compatibilità predefinito scarica i frame nella RAM. Non è a copia zero. I lavoratori extra possono aumentare la contesa. I profili non modificano mai le impostazioni di gioco né riducono silenziosamente la risoluzione o gli FPS.

## Audio

Per impostazione predefinita il microfono è disattivato. Abilitalo esplicitamente in Audio e scegli il suo dispositivo. L'audio del desktop cattura l'endpoint di output selezionato, non solo un gioco. Usa le cuffie.

Abilita il routing dell'audio misto e delle tracce indipendenti secondo necessità. Il file risultante contiene vere e proprie tracce separate. RNoise, gate, espansore, compressore, limitatore e guadagno possono essere abilitati e riordinati. Potrebbe persistere il rumore della tastiera.

Utilizza il test del microfono di otto secondi e riproduci il test salvato per giudicare la chiarezza. Il monitoraggio in tempo reale delle cuffie e l'audio di gioco specifico del processo non sono implementati.

## Controlli di registrazione

F9 avvia o interrompe la registrazione. F10 mette in pausa o riprende. F11 disattiva il microfono. F7 mostra o nasconde l'applicazione. F8 salva il buffer di riproduzione solo video avviato esplicitamente. Le scorciatoie in conflitto possono essere modificate in Tasti di scelta rapida.

Trascina la barra del titolo personalizzata per spostare la finestra. Fare doppio clic per ingrandirlo. La chiusura durante una registrazione la ferma e la finalizza in modo sicuro. L'illustrazione originale non è un'anteprima dal vivo.

## File e ripristino

MKV è l'impostazione predefinita. MP4 e MP4 frammentato vengono finalizzati dopo l'arresto. L'audio viene catturato nei sidecar PCM e aggiunto dopo l'arresto. Le sessioni lunghe possono richiedere tempo per essere finalizzate.

Ciascuna sorgente audio stereo abilitata utilizza circa 1,38 GB all'ora in PCM temporaneo, oltre al video e alle copie intermedie. Le cartelle di ripristino conservano gli originali. Controllare la registrazione completata prima di rimuovere manualmente la cartella della sessione.

Utilizza le registrazioni per aprire file, convertire MKV in MP4 senza ricodificare il video o recuperare sessioni non completate. Le registrazioni originali non vengono eliminate automaticamente.

## Limiti e verifica

Replay attualmente salva solo il video. HDR, ottimizzazione adattiva, isolamento dell'audio del processo, monitoraggio dal vivo e ripristino automatico del ripristino della GPU sono incompleti. Si stima l'allineamento audio/video all'avvio; controlla la sincronizzazione labiale prima di registrazioni importanti.

I contenuti protetti, alcuni giochi esclusivi a schermo intero e le finestre ridotte a icona potrebbero interrompere la distribuzione dei frame. Alt+Tab mantiene aperta la sessione, ma non può rimuovere le restrizioni del sistema operativo.

Non è stato stabilito alcun vantaggio in termini di prestazioni rispetto a OBS o Xbox Game Bar. Consulta il protocollo di riferimento e lo stato di implementazione per i test riproducibili e il lavoro rimanente.

## Nota di traduzione

Questo pacchetto linguistico è stato assistito da un computer e non ha ancora ricevuto una revisione completa da parte di un madrelingua. La diagnostica tecnica Windows e FFmpeg mantengono la loro formulazione originale.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
