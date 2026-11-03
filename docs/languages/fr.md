# Potato Recorder — Français

**Guide d'utilisation**

Un petit studio natif Windows pour l'enregistrement de gameplay.

## Commencer

Installez FFmpeg 8.1 ou plus récent et le ffprobe correspondant. Mettez-les sur PATH ou à côté de l'application. L'application ne télécharge pas d'outils ni ne télécharge d'enregistrements.

Ouvrez Paramètres pour sélectionner votre langue. Dans Enregistrement, sélectionnez un affichage ou une fenêtre de jeu. Windows Graphics Capture prend en charge les fenêtres et les écrans ; DXGI prend en charge les affichages.

## Vidéo et performances

Commencez avec 1280 × 720 à 60 FPS sur les graphiques intégrés, puis mesurez les temps d'image du jeu. La valeur par défaut reste la résolution source. Il s'agit d'une recommandation et non d'une garantie de performance.

Teste automatiquement les encodeurs matériels réels. Quantification des changements de qualité ; la vitesse change l'effort de l'encodeur. Les fréquences d'images fractionnaires exactes sont prises en charge jusqu'à 120 FPS. La livraison réelle à la source peut être inférieure.

Le transfert de compatibilité par défaut télécharge les images dans la RAM. Ce n’est pas une copie zéro. Les travailleurs supplémentaires peuvent accroître les conflits. Les profils ne modifient jamais les paramètres de jeu ni ne diminuent silencieusement la résolution ou le FPS.

## Audio

Le microphone est désactivé par défaut. Activez-le explicitement dans Audio et choisissez son appareil. Le son du bureau capture le point final de sortie sélectionné, pas seulement un jeu. Utilisez des écouteurs.

Activez le routage audio mixte et piste indépendante selon vos besoins. Le fichier résultant contient de véritables pistes séparées. RNNoise, gate, expander, compresseur, limiteur et gain peuvent être activés et réorganisés. Le bruit du clavier peut persister.

Utilisez le test du microphone de huit secondes et lisez le test enregistré pour juger de la clarté. La surveillance des écouteurs en direct et l'audio du jeu spécifique au processus ne sont pas implémentés.

## Contrôles d'enregistrement

F9 démarre ou arrête l'enregistrement. F10 fait une pause ou reprend. F11 coupe le microphone. F7 affiche ou masque l'application. F8 enregistre le tampon de relecture vidéo uniquement démarré explicitement. Les raccourcis en conflit peuvent être modifiés dans les raccourcis clavier.

Faites glisser la barre de titre personnalisée pour déplacer la fenêtre. Double-cliquez dessus pour maximiser. La fermeture pendant un enregistrement l'arrête et le finalise en toute sécurité. L'illustration source n'est pas un aperçu en direct.

## Fichiers et récupération

MKV est la valeur par défaut. MP4 et MP4 fragmenté sont finalisés après l'arrêt. L'audio est capturé dans les side-cars PCM et ajouté après l'arrêt. Les longues sessions peuvent prendre du temps à finaliser.

Chaque source audio stéréo activée utilise environ 1,38 Go par heure en PCM temporaire, en plus des copies vidéo et intermédiaires. Les dossiers de récupération conservent les originaux. Vérifiez l'enregistrement terminé avant de supprimer manuellement son dossier de session.

Utilisez les enregistrements pour ouvrir des fichiers, remuxer MKV en MP4 sans réencodage vidéo ou récupérer des sessions inachevées. Les enregistrements originaux ne sont pas automatiquement supprimés.

## Limites et vérification

La relecture enregistre actuellement uniquement la vidéo. Le HDR, l'optimisation adaptative, l'isolation audio des processus, la surveillance en direct et la récupération automatique de la réinitialisation du GPU sont inachevés. L’alignement audio/vidéo au démarrage est estimé ; vérifiez la synchronisation labiale avant les enregistrements importants.

Le contenu protégé, certains jeux exclusifs en plein écran et les fenêtres réduites peuvent cesser de fournir des images. Alt+Tab maintient la session ouverte, mais ne peut pas supprimer les restrictions du système d'exploitation.

Aucun avantage en termes de performances par rapport à OBS ou Xbox Game Bar n'a été établi. Consultez le protocole de référence et l’état de mise en œuvre pour connaître les tests reproductibles et le travail restant.

## Note de traduction

Ce module linguistique a été assisté par machine et n'a pas encore reçu une révision complète par un locuteur natif. Les diagnostics techniques Windows et FFmpeg conservent leur formulation d'origine.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
