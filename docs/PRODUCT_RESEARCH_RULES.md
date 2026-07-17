# Belegregeln für die Produktrecherche

Diese Regeln entstanden aus einem gescheiterten ersten Versuch. Das Ergebnis war
1022 Zeilen, inhaltlich nicht einmal komplett falsch, und trotzdem wertlos und
verbrannt. Der Grund steht hier, damit der nächste Versuch nicht daran stirbt.

## Was tatsächlich passiert ist

Ein Agent zitierte elfmal `Blizzard wörtlich`. Das Dokument enthielt dreißig
Quellenlinks, davon **null** auf eine Blizzard-Domain. Eines der Zitate liess
sich nur auf einem Tapatalk-Forum einer Custom-Map-Community wiederfinden. Der
Agent behauptete außerdem, er habe `die 99 Zeilen der Datei vollständig
gelesen`. Diese Datei existiert nirgends auf der Platte.

Der Text selbst war vermutlich echt, wahrscheinlich aus Blizzards
`CustomKeyInfo.txt`. Erfunden war nicht der Text, erfunden war die **Autorität**.
Das ist Quellenwäsche, und sie ist der gefährlichste Fehler überhaupt, weil das
Ergebnis plausibel klingt und nur der Auftraggeber sie aufdecken kann.

## Die Regeln

1. **Ohne abrufbare Quelle ist eine Aussage ungültig.** Nicht schwach, nicht
   unsicher, sondern ungültig. Sie kommt nicht ins Dokument. Eine ehrliche Lücke
   ist immer besser als ein unbelegter Satz.

2. **Es gibt genau vier Quellenklassen, und jede Aussage trägt ihre Klasse
   sichtbar.** Ohne Klasse keine Aufnahme.
   - `blizzard-offiziell`: eine Seite oder Datei von Blizzard selbst. Die URL
     muss auf eine Blizzard-Domain zeigen, oder die Datei muss mit Pfad
     benannt sein und auf der Platte existieren.
   - `community`: Forum, Reddit, Liquipedia, Hive, YouTube, gaming-tools. Das ist
     eine gültige Klasse, aber sie ist **nie** dasselbe wie Blizzard.
   - `repo-code`: eine Datei im Repo, mit Pfad und Zeilennummer. Muss existieren.
   - `auftraggeber`: vom Auftraggeber bestätigte Tatsache. Gilt als wahr, ist
     aber als solche zu kennzeichnen und nicht als Recherche auszugeben.

3. **Quellenwäsche ist verboten.** Ein Forum, das Blizzard zitiert, ist
   `community`, niemals `blizzard-offiziell`. Die Klasse richtet sich nach dem
   Ort, an dem du es gefunden hast, nicht nach dem, was der Ort über sich
   behauptet. Wer eine Blizzard-Aussage will, muss sie bei Blizzard finden.

4. **`Ich habe die Datei gelesen` gilt nur, wenn die Datei existiert.** Nenne
   immer den vollständigen Pfad. Ein Prüfer muss ihn öffnen können. Eine
   erfundene Leseerfahrung ist eine Lüge, kein Flüchtigkeitsfehler.

5. **Ein Zitat braucht das Zitat.** Wörtlicher Text, dazu die URL oder der Pfad,
   dazu Autor und Datum, soweit ermittelbar. Fehlt der wörtliche Text, ist es
   kein Zitat, sondern eine Erinnerung, und Erinnerungen sind hier wertlos.

6. **Nichts aus dem Gedächtnis.** Auch nicht, wenn es offensichtlich stimmt.
   Wenn es stimmt, gibt es dafür eine Quelle. Wenn es keine gibt, gehört es in
   die Lücken.

7. **Die Lücken gehören ins Dokument.** Was sich nicht belegen liess, wird
   benannt, nicht weggelassen und schon gar nicht überbrückt. Eine benannte
   Lücke ist ein Arbeitsauftrag, ein überbrückte Lücke ist eine Falle.

## Die Zielgruppe kennt ihr Handwerk

Ein zweiter Fehler des ersten Versuchs. Der Agent konstruierte eine Produktlücke
daraus, dass der Editor die Gameplay-Option `Custom Keys` nicht erwähnt. Diese
Option lebt in `War3Preferences.txt`, einer Datei, die dieses Produkt nicht
anfasst. Und wer einen Hotkey-Editor sucht, kämpft seit Jahren mit CustomKeys und
hat diesen Schalter längst umgelegt.

Unterstelle der Zielgruppe niemals Unwissen, das sie nicht hat. Diese Leute
schreiben seit zwanzig Jahren Rawcodes von Hand in Textdateien. Jede Aussage, die
sie für Anfänger hält, ist falsch, auch wenn sie sich aus einer echten Quelle
herleiten lässt.
