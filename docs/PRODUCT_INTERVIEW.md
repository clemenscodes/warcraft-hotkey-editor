# Interviewplan für docs/PRODUCT.md

## Warum ein Interview und keine Recherche

Zwei Rechercheversuche sind gescheitert. Der erste an Quellenwäsche, der zweite
daran, dass er zwar sauber belegt war, aber jede Aussage sofort selbst
einschränken musste und damit nichts mehr behauptet hat. Die Ursache war beide
Male dieselbe und lag im Zuschnitt, nicht in der Ausführung.

Es wurde versucht, aus dem Internet herzuleiten, warum dieses Produkt existiert.
Dazu gibt es im Netz fast nichts, nämlich genau eine echte Blizzard-Quelle. Also
blieb nur Hedging, und ein Dokument, das jede Aussage relativiert, ist als
Pflichtlektüre wertlos.

Der Domänenexperte ist der Auftraggeber. Das Produkt wurde mit Back2Warcraft
entwickelt. Für Zweck, Zielgruppe und die klassischen Probleme ist er die Quelle,
nicht ein Forenpost von 2019. Seine Aussagen sind Klasse `auftraggeber` und
brauchen keinen externen Beleg, weil es keine bessere Quelle gibt.

## Die drei Wissensquellen, sauber getrennt

Das war der Zuschnittfehler. Richtig ist diese Aufteilung.

**Der Code weiß es.** Feature-Katalog, Routen, Dialoge, die Lücken zwischen
Desktop und Mobil, welche Domänenoperationen existieren. Das wird gelesen, nicht
gefragt. Der zweite Durchlauf hat hier bereits gute Arbeit geliefert, 41 Features
mit Pfaden und elf benannte Mobil-Lücken. **Dieser Teil ist brauchbar und wird
übernommen.**

**Der Auftraggeber weiß es.** Zweck, Zielgruppe, die klassischen Probleme, der
Arbeitsablauf, der CASC-Vorteil, die Rolle von Back2Warcraft, was gut und was
schlecht ist. Wird gefragt, nicht recherchiert, und ohne Einschränkung
aufgeschrieben.

**Das Netz weiß es.** Ausschließlich die Konkurrenz, also fremde Projekte, die
tatsächlich öffentlich dokumentiert sind. Hier und nur hier gelten die
Belegregeln aus `PRODUCT_RESEARCH_RULES.md` in voller Härte.

## Die Fragen

Kurz gehalten und beantwortbar. Keine Fragen, deren Antwort im Code steht.

### Zweck und die klassischen Probleme

1. Beschreib den Weg, den ein Spieler ohne diesen Editor gehen musste. Was genau
   hat die Tage bis Wochen gekostet, und woran ist man dabei konkret gescheitert.
2. Du sagst, man endete selbst nach Wochen mit zig Konflikten. Was für Konflikte
   sind das, und warum entstehen sie überhaupt.
3. Was ist der Moment, in dem ein Spieler merkt, dass er ein Werkzeug braucht.
4. Was kann dieses Produkt, was vorher schlicht unmöglich war, nicht nur mühsam.

### Zielgruppe

5. Wer benutzt das. Wenn es mehrere Gruppen gibt, welche und was unterscheidet
   sie in dem, was sie wollen.
6. Wie groß ist die Gruppe realistisch. Die Recherche fand rund 19.000 gewertete
   1v1-Accounts, stimmt diese Größenordnung.
7. Was kann die Zielgruppe bereits, das man ihr niemals erklären darf. Der erste
   Versuch hat hier eine Produktlücke erfunden, weil er ihnen Unwissen
   unterstellt hat.
8. Gibt es Leute, für die das Produkt ausdrücklich nicht gebaut ist.

### Der Arbeitsablauf, der Kern

9. Führ mich durch die 30 bis 60 Minuten. Was tut man zuerst, was danach, wann
   ist man fertig.
10. Der Kern ist das Verschieben der Buttons pro Unit. Wie viele Units macht man
    in einer Sitzung, und in welcher Reihenfolge geht man sie durch.
11. Woher weiß man, dass eine Unit fertig ist, und woher, dass das Template
    fertig ist.
12. Was ist der nervigste Teil daran, heute, mit dem Editor.

### Der CASC-Vorteil

13. Beschreib, was bei einem Blizzard-Patch passiert, bei uns und bei einem
    handgepflegten Werkzeug.
14. Merkt ein Nutzer das überhaupt, oder ist es ein unsichtbarer Vorteil.

### Back2Warcraft

15. Wie kam die Kooperation zustande, und was steckt konkret an ihrem Wissen im
    Produkt. Die einzige Spur im Code sind drei NEO-Templates von sieben.
16. Was hätte ohne sie gefehlt.

### Mobil

17. Du sagst, mobil wird im Zug, an der Bushaltestelle und im Wartezimmer
    geplant. Was genau tut man dort, und was hebt man sich für den Desktop auf.
18. Die Recherche fand elf Mobil-Lücken, darunter kein PWA-Manifest, also lädt
    die App ohne Netz nicht, und kein Onboarding auf dem Telefon. Welche davon
    sind wirklich schmerzhaft und welche egal.

### Konkurrenz

19. Was weißt du über jcfieldsdev, das man im Netz nicht sieht.
20. Warum hat es niemand vor uns richtig gelöst.

### Gut und schlecht

21. Worauf bist du beim aktuellen Stand stolz.
22. Was ist heute die größte Schwäche des Produkts.
23. Woran misst du, ob das Produkt erfolgreich ist.

## Wie das Dokument danach entsteht

Antworten werden **als Aussage** aufgeschrieben, nicht als Hypothese, und als
Klasse `auftraggeber` gekennzeichnet. Keine Warnboxen, keine Selbsteinschränkung,
kein Prozessschutt über gescheiterte Rechercheversuche. Das gehört hierher und in
`PRODUCT_RESEARCH_RULES.md`, nicht in ein Produktdokument.

Der Feature-Katalog kommt aus dem Code und wird beim Schreiben stichprobenartig
gegen die Pfade geprüft.

Nur die Konkurrenzanalyse wird recherchiert, mit Quellenklasse und Link, nach den
harten Regeln.

Wenn eine Antwort fehlt, bleibt die Frage offen und wird als offene Frage
benannt. Nicht überbrücken, nicht erraten.
