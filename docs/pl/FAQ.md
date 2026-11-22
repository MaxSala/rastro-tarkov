<p align="center">
  <a href="https://maxsala.github.io/rastro-tarkov/pl/docs/faq/">Strona</a> · <a href="../../README.pl.md">Powrót do README</a> · <a href="FEATURES.md">Funkcje</a> · <a href="GUIDE.md">Przewodnik</a> · <b>Pytania i bezpieczeństwo</b>
</p>

# Pytania i bezpieczeństwo

## Czy to bezpieczne?

Rastro to mały, przenośny program. Oto co to oznacza i jak możesz to sprawdzić.

- **Bez instalatora i uprawnień administratora.** To jeden plik `.exe`. Nie instaluje usług ani sterowników, nie uruchamia się z Windowsem i nie rusza plików gry. Ustawienia i pobrane mapy trzyma w jednym folderze (zobacz [Gdzie są twoje dane](GUIDE.md#gdzie-są-twoje-dane)), a usunięcie tego folderu i pliku `.exe` usuwa go całkowicie.
- **Open source.** Cały kod jest w tym repozytorium. Każdy może go przeczytać.
- **Budowany publicznie.** Każdą wersję budują serwery GitHuba prosto z kodu źródłowego, a nie czyjś PC. Każdy build widać w zakładce [Actions](https://github.com/MaxSala/rastro-tarkov/actions), a strona wydania podaje SHA-256 pliku, żebyś mógł sprawdzić, że masz dokładnie ten plik:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Niczego nie wysyła.** Bez telemetrii, reklam i konta. Łączy się z internetem tylko wtedy, gdy o to poprosisz, a pełna lista połączeń jest w sekcji [Prywatność](#prywatność-i-sieć).

**„System Windows ochronił ten komputer".** Przy pierwszym uruchomieniu Windows SmartScreen najpewniej pokaże niebieskie okno z informacją, że zatrzymał nierozpoznaną aplikację. Dzieje się tak z każdym nowym programem bez płatnego certyfikatu, a nie dlatego, że z plikiem jest coś nie tak. Kliknij **Więcej informacji**, a potem **Uruchom mimo to**. Windows zapamięta twój wybór. Niektóre antywirusy też podchodzą nieufnie do nowych, niepodpisanych programów. Jeśli twój oznaczy Rastro, możesz go przywrócić i dodać wyjątek, wrzucić plik na [VirusTotal](https://www.virustotal.com/) dla drugiej opinii i zgłosić fałszywy alarm producentowi antywirusa.

> [!IMPORTANT]
> Rastro to nieoficjalny projekt fanowski, niezwiązany z Battlestate Games ani tarkov.dev i nieautoryzowany przez nich. Jest zbudowany tak, żeby trzymać się z dala od gry: czyta tylko pliki z dysku i nigdy nie wchodzi w interakcję z procesem gry. Mimo to korzystanie z narzędzi firm trzecich z Escape from Tarkov odbywa się na twoją odpowiedzialność w świetle warunków korzystania z usług Battlestate Games.

## Pytania

**Czy mogę dostać bana?**
Rastro nigdy nie czyta pamięci gry, nie dotyka procesu gry i nie wysyła do niej żadnych klawiszy. Czyta nazwy zrzutów i logi już po tym, jak gra je zapisała, czyli te same pliki, które możesz sam otworzyć w Eksploratorze. Nie możemy jednak mówić w imieniu Battlestate Games, więc przeczytaj uwagę w [Czy to bezpieczne?](#czy-to-bezpieczne)

**Czy działa w PvE?**
Tak. Wykrywanie rajdu było testowane na logach PvE, a Rastro pobiera ceny i dane PvE, gdy wykryje PvE.

**Czy potrzebuje internetu?**
Tylko do pobierania map i cen, sprawdzania aktualizacji (jeśli na to pozwolisz) i gry w drużynie. Cała reszta działa offline.

**Dlaczego zrzuty ekranu?**
To jedyne miejsce, w którym gra zapisuje twoją dokładną pozycję na dysku. Kilka sekund opóźnienia w zamian za to, że gra pozostaje nietknięta.

**Linux albo macOS?**
Rastro buduje się i działa na Linuksie; macOS nie był testowany. Sam Tarkov działa tylko na Windowsie, więc plik do pobrania jest tylko dla Windows. Zobacz [Budowanie ze źródeł](GUIDE.md#budowanie-ze-źródeł).

## Prywatność i sieć

Rastro nie ma konta, telemetrii ani reklam. Interfejs rozmawia tylko z samą aplikacją pod adresem `127.0.0.1`, chroniony losowym kluczem, który zmienia się przy każdym uruchomieniu. Wszystko, co czyta z dysku, zostaje na twoim PC. Reszta dzieje się tylko na twoją prośbę:

| Kiedy | Łączy się z |
|---|---|
| Zawsze | Niczym poza twoim PC |
| Pobierasz mapy z tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` i awaryjnie `api.tarkov.dev` |
| Pobierasz lub odświeżasz ceny | `json.tarkov.dev`, plus `assets.tarkov.dev` dla ikon przedmiotów |
| Pobierasz lub odświeżasz zadania | `json.tarkov.dev` i awaryjnie `api.tarkov.dev`, plus `assets.tarkov.dev` dla obrazków |
| Importujesz paczkę map z adresu URL | Tylko z adresem, który wpiszesz (tylko HTTPS, nigdy adresy wewnętrzne) |
| Tworzysz pokój albo dołączasz do drużyny | Bezpośrednio z PC kolegów |
| Raz na uruchomienie, jeśli zgodziłeś się na sprawdzanie aktualizacji (albo po kliknięciu *Sprawdź teraz*) | `api.github.com`, żeby odczytać numer najnowszej wersji. Nic o tobie nie jest wysyłane. |

Każdy serwer pobierania jest na stałej liście dozwolonych, sprawdzanej także przy przekierowaniach, a wszystko, co pobrane, jest weryfikowane przed użyciem. Pełny model zagrożeń jest w [docs/SECURITY.md](../SECURITY.md) (po angielsku).

## Znane ograniczenia

- Twoja pozycja aktualizuje się tylko po zrobieniu zrzutu ekranu. Tak ma być: to jedyne miejsce, w którym gra zapisuje twoją pozycję.
- Labs nie ma mapy SVG na tarkov.dev. Rastro rozpoznaje Labs, ale nie pokaże go, dopóki nie zaimportujesz własnej paczki map.
- Wykrywanie rajdu zostało sprawdzone na logach PvE. W PvP powinno działać tak samo, ale nie zostało to potwierdzone na prawdziwych logach.
- Tłumaczenia hiszpańskie i angielskie są sprawdzone. Pozostałych 14 nie przejrzeli jeszcze native speakerzy. Poprawki są bardzo mile widziane, tutaj albo na [Discordzie](https://discord.gg/H4pRxGBGDb).
- Po zmianie języka albo trybu gry (PvP/PvE) trzeba ponownie pobrać mapy i ceny, żeby zobaczyć nowe nazwy.
- Niektóre szczegóły techniczne w komunikatach o błędach i w dzienniku są po angielsku niezależnie od wybranego języka.

## Niekomercyjny projekt

Rastro to hobbystyczny projekt dla społeczności. Jest darmowy, nie ma płatnej wersji, nie wyświetla reklam i nie zbiera danych. Tak zostanie, z wyboru i dlatego, że mapy tarkov.dev, na których się opiera, są licencjonowane wyłącznie do użytku niekomercyjnego.
