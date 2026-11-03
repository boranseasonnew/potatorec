# Potato Recorder — Suomi

**Käyttöopas**

Pieni natiivi Windows-studio pelin tallentamiseen.

## Aloittaminen

Asenna FFmpeg 8.1 tai uudempi ja vastaava ffprobe. Aseta ne PATH:lle tai sovelluksen viereen. Sovellus ei lataa työkaluja tai tallenteita.

Valitse kieli avaamalla Asetukset. Valitse Tallennus-kohdassa näyttö tai peliikkuna. Windows Graphics Capture tukee ikkunoita ja näyttöjä; DXGI tukee näyttöjä.

## Video ja esitys

Aloita 1280 × 720 nopeudella 60 FPS integroidulla grafiikalla ja mittaa sitten pelin kuva-ajat. Oletusarvo pysyy lähteen resoluutiona. Tämä on suositus, ei suoritustakuu.

Auto testaa todellisia laitteistokoodareita. Laatu muuttaa kvantisointia; nopeus muuttaa kooderin vaivaa. Tarkkoja kuvanopeuksia tuetaan 120 FPS asti. Todellinen lähteen toimitus voi olla pienempi.

Yhteensopivuuden oletussiirto lataa kehykset RAM-muistiin. Se ei ole nollakopio. Ylimääräiset työntekijät voivat lisätä kiistaa. Profiilit eivät koskaan muuta peliasetuksia tai alenna äänettömästi resoluutiota tai FPS:ää.

## Ääni

Mikrofoni on oletuksena pois päältä. Ota se käyttöön erikseen Audiossa ja valitse sen laite. Työpöytäääni kaappaa valitun lähtöpisteen, ei vain yhtä peliä. Käytä kuulokkeita.

Ota Mixed Audio ja Independent Track -reititys käyttöön tarvittaessa. Tuloksena oleva tiedosto sisältää todellisia erillisiä raitoja. RNNoise, gate, Expander, kompressori, rajoitin ja vahvistus voidaan ottaa käyttöön ja järjestää uudelleen. Näppäimistön kohina saattaa jäädä.

Käytä kahdeksan sekunnin mikrofonitestiä ja toista tallennettu testi arvioidaksesi selkeyttä. Suoraa kuulokkeiden valvontaa ja prosessikohtaista peliääntä ei ole otettu käyttöön.

## Tallennuksen säätimet

F9 aloittaa tai lopettaa tallennuksen. F10 keskeyttää tai jatkaa. F11 mykistää mikrofonin. F7 näyttää tai piilottaa sovelluksen. F8 tallentaa eksplisiittisesti aloitetun vain videon toistopuskurin. Ristiriitaisia ​​pikanäppäimiä voidaan muuttaa pikanäppäimissä.

Siirrä ikkunaa vetämällä mukautettua otsikkopalkkia. Kaksoisnapsauta sitä suurentaaksesi. Sulkeminen tallennuksen aikana pysähtyy ja viimeistelee sen turvallisesti. Lähdekuva ei ole live-esikatselu.

## Tiedostot ja palautus

MKV on oletusarvo. MP4 ja pirstoutunut MP4 viimeistellään pysäytyksen jälkeen. Ääni tallennetaan PCM-sivuvaunuihin ja lisätään pysähtymisen jälkeen. Pitkien istuntojen viimeistely voi viedä aikaa.

Jokainen käytössä oleva stereoäänilähde käyttää noin 1,38 Gt tunnissa väliaikaisessa PCM:ssä video- ja välikopioiden lisäksi. Palautuskansiot säilyttävät alkuperäiset. Tarkista valmis tallennus ennen kuin poistat sen istuntokansion manuaalisesti.

Tallenteiden avulla voit avata tiedostoja, muokata MKV:ta MP4:ksi ilman videon uudelleenkoodausta tai palauttaa keskeneräiset istunnot. Alkuperäisiä tallenteita ei poisteta automaattisesti.

## Rajoitukset ja todentaminen

Toisto tallentaa tällä hetkellä vain videon. HDR, adaptiivinen optimointi, prosessin äänen eristäminen, live-seuranta ja automaattinen GPU-nollauspalautus ovat keskeneräisiä. Käynnistyksen audio/video kohdistus on arvioitu; tarkista huulten synkronointi ennen tärkeitä tallennuksia.

Suojattu sisältö, jotkin eksklusiiviset koko näytön pelit ja pienennetyt ikkunat saattavat lakata toimittamasta kehyksiä. Alt+Tab pitää istunnon auki, mutta ei voi poistaa käyttöjärjestelmän rajoituksia.

Suorituskykyetua OBS:ään tai Xbox Game Bariin verrattuna ei ole vahvistettu. Katso benchmark-protokollasta ja toteutustilasta toistettavat testit ja jäljellä oleva työ.

## Käännöshuomautus

Tämä kielipaketti oli koneavusteinen, eikä se ole vielä saanut täydellistä äidinkielenään arviota. Tekninen Windows ja FFmpeg-diagnostiikka säilyttävät alkuperäisen sanamuotonsa.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
