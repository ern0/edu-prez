# Hogyan programoztunk mi MUMPS-ot a '80-as évek végén?

## Bevezető

\- A 256-byte intróidhoz milyen framework-öt használsz?
Vagy csak simán OpenGL-t? - kérdezte tőlem egyszer egy
fiatal kolléga.
Elmagyaráztam neki,
hogy 256 byte-ba
kb. már az OpenGL kép megnyitása se férne bele,
ezért direktben rakosgatjuk a pixeleket a VGA vidomemóriába,
ráadásul MS-DOS alatt,
mivel
egyrészt
a `.COM` formátumnak nincs fejléce,
így mind a 256 byte-ot nettó kódra lehet elkölteni,
másrészt
a 16-bites 8086/80286 utasítások jóval rövidebbek
mint a 32 vagy 64 bitesek.

A MUMPS is hasonlóan nehezen érthető mai ésszel.
Megalkotásakor,
*1966-ban*
a rendszer tervezőinek
olyan korlátokat kellett leküzdeni,
amiket ma már igazából meg sem értünk,
nemhogy felmerülnének.
Ezért a rendszer
elemeit, funkcióit, az egyes megoldásokat
mindig ennek a tükrében kell értékelni.

### A nyelv: interpteter

A MUMPS **interpretált** nyelv.
Az interpreter egy program,
ami beolvassa a tárban lévő programszöveget,
és végrehajtja az egyes utásításokat.

Ez lassabb, mint amikor a programszöveget (forráskódot)
egy *compiler* lefordítja gépi kódra,
és az így kapott programot közvetlenül futtatjuk a gépen,
viszont maga a fordítás erőforrás- és időigényes.
A MUMPS esetében,
ahol a programok többsége adatbázist kezelő alkalmazás,
ez nem gyorsítana sokat,
mivel
az idő nagy részét
nem a program által végzett számolás viszi el,
hanem az adatbázis-műveletek.

> A modern MUMPS-ok már JIT compilert használnak:
> a program futása előtt
> a forráskód lefordul gépi kódra.

### A nyelv: okenizálás helyett

Az interpretált nyelvek esetén
nem az utasítások, hanem tokenjeik kerülnek tárolásra.
Például ha van egy Commodore BASIC programunk:
```
10 PRINT "HELLO"
```
a memóriában nem az lesz,
hogy "PRINT" (5 byte),
hanem a `PRINT` utasítás tokenje: `153` (`$99`),
ami csak 1 byte.
A Commodore BASIC a program írásakor
kikeresi a kulcsszavakhoz való tokeneket,
és azt tárolja el a programsorban,
amivel azt nyerjük,
hogy
futás közben nem kell már kulcsszavakat keresgélni,
csak az 1 byte-os tokeneket kell interpretálni.
Még annyi teendő van a tokenekkel, hogy
listázáskor a token kódok helyett
utasítások neveit kell kiírni.

A MUMPS nem tokenizál,
mégis csak 1 byte-os utasításokat interpretál.
A trükk egyszerű:
minden utasítás különböző betűvel kezdődik,
így az első betű maga a token,
a többit ki sem kell írni
(nem is javasolt, mert csak a helyet foglalja).

```
FIZZBUZZ ; FizzBuzz in MUMPS, using short instructions
         ;
         S I=1
         ;
NEXT     I I#15=0 W "FizzBuzz",! G INC
         I I#3=0 W "Fizz",! G INC
         I I#5=0 W "Buzz",! G INC
         W I,!
         ;
INC      S I=I+1
         I I<100 G NEXT
         Q
```

Az egybetűs utasításokat meglepően könnyű megszokni,
én egyszer kaptam egy kis feladatot,
10-soros programot kellett írnom,
viccből teljes utasításokkal írtam meg.
Mögém állt a fél Számítástechnikai osztály,
ilyet még nemigen láttak,
aztán a Józsi, a vezető programozó megszólalt:

\- Ez nagyon szép, de mégegyszer ne csinálj ilyet,
nagyon zavaró.

```
FIZZBUZZ ; FizzBuzz in MUMPS, using full instructions
         ;
         SET I=1
         ;
NEXT     IF I#15=0 WRITE "FizzBuzz",! GOTO INC
         IF I#3=0 WRITE "Fizz",! GOTO INC
         IF I#5=0 WRITE "Buzz",! GOTO INC
         WRITE I,!
         ;
INC      SET I=I+1
         IF I<100 GOTO NEXT
         QUIT
```

> A modern MUMPS-ok esetében javasolt a hosszú utasításnevek kiírása,
és a GOTO utasítás minimális használata is.

### A nyelv: operátor-precedencia

Egyszer egy órácskát eljátszottam egy egyszerű képlettel,
sehogy sem jött ki az az eredmény, amire vágytam.
Nagyjából így nézett ki:
```
> SET A=2,B=3
> WRITE A*10+B*5
115                ; 35 helyett
```

Karcsi, a szervezési osztály vezetője mögém állt,
nézte, mit szenvedek:

\- Tegyél zárójelet!

```
> SET A=2,B=3
> WRITE A*10+(B*5)
35
```

Ez is csak az optimalizálás végett van így:
az operátor-precedencia hanyagolása miatt
a kiértékelés
gyorsabb,
kevesebb memória kell hozzá, és
valamivel kisebb az interpreter programja is.

### Az adatbázis: fa-struktúra

