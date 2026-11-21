<p align="center">
  <a href="../../README.pl.md">Powrót do README</a> · <a href="FEATURES.md">Funkcje</a> · <b>Przewodnik</b> · <a href="FAQ.md">Pytania i bezpieczeństwo</a>
</p>

# Przewodnik

## Instalacja

1. **[Pobierz `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Ten link zawsze pobiera najnowszą wersję.
2. Wrzuć go, gdzie chcesz: na pulpit, do folderu w Dokumentach, gdziekolwiek. Nie potrzebuje żadnych innych plików obok.
3. Kliknij dwukrotnie. Jeśli Windows pokaże niebieskie ostrzeżenie, zobacz [Czy to bezpieczne?](FAQ.md#czy-to-bezpieczne)

Wymaga 64-bitowego Windows 10 lub 11.

### Pierwsze uruchomienie

Za pierwszym razem otwiera się krótki przewodnik:

1. **Folder zrzutów ekranu.** Rastro sam znajduje `Documents\Escape from Tarkov\Screenshots`, także wtedy, gdy Windows przeniósł Dokumenty do OneDrive. Jeśli nie znajdzie, wklej ścieżkę, a pokaże, ile zrzutów ze współrzędnymi widzi. Tutaj możesz też włączyć wykrywanie rajdu.
2. **Mapy.** Pobierz mapy społeczności z tarkov.dev (zalecane), zaimportuj paczkę map albo wypróbuj mapę demo.
3. Gotowe. Zrób zrzut ekranu w grze, a na mapie pojawi się twoja strzałka.

Żeby korzystać z Rynku, tabeli Amunicji i nazw kluczy, otwórz raz zakładkę Rynek i kliknij *Pobierz ceny*.

### Aktualizacja

Jeśli pozwoliłeś na sprawdzanie aktualizacji, przy nowej wersji na górnym pasku pojawi się przycisk prowadzący do strony pobierania. W przeciwnym razie pobierz nowy `.exe` z [linku powyżej](#instalacja), zamknij Rastro i używaj nowego pliku zamiast starego. Ustawienia, punkty, mapy i ceny są trzymane [osobno](#gdzie-są-twoje-dane), więc nic nie przepadnie.

Po aktualizacji Rastro pokazuje, co się zmieniło. Pełna historia jest w *Ustawienia → O programie → Co nowego* i w [CHANGELOG.md](../../CHANGELOG.md). Jeśli aktualizacja przynosi nowe dane map, użyj *Aktualizuj z tarkov.dev* w panelu Mapy.

### Skróty

| Akcja | Jak |
|---|---|
| Zaktualizuj swoją pozycję | Zrób zrzut ekranu w grze |
| Zmień sekcję (Mapa, Rynek, Amunicja, Zadania, Kryjówka) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4`, `Ctrl+5` |
| Paleta poleceń | `Ctrl+K` |
| Panele ewakuacji, punktów, drużyny, map i zadań | `1`, `2`, `3`, `4`, `5` |
| Nowy rajd (wyczyść ślad i znaczniki) | `N` |
| Pokaż całą mapę / podążaj za pozycją | `F` / `G` |
| Przybliżanie | `+` / `-` albo kółko myszy |
| Dodaj punkt, prowadź do miejsca | Prawy klik na mapie |
| Pinguj drużynę | `Shift` + klik |
| Szukaj na Rynku | `/` |
| Ustawienia | `,` |

### Gdzie są twoje dane

Ustawienia, twoje punkty, postęp zadań, pobrane mapy oraz pamięć podręczna cen i zadań są w jednym folderze, oddzielnie od pliku `.exe`:

| System | Folder |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

W *Ustawienia → Dziennik* jest przycisk, który go otwiera.

### Jeśli coś nie działa

- **Otwiera się w przeglądarce zamiast we własnym oknie.** Rastro rysuje okno za pomocą Microsoft WebView2. Jest on częścią Windows 10 i 11, ale jeśli został usunięty, Rastro otwiera się w przeglądarce. Zainstaluj [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/), a okno wróci.
- **Strona błędu mówi, że port jest zajęty.** Pewnie działa jeszcze inna kopia Rastro. Zamknij ją z paska zadań albo z Menedżera zadań (szukaj `rastro.exe`) i uruchom ponownie.
- **Moja pozycja w ogóle się nie zmienia.** Sprawdź, czy folder zrzutów w *Ustawienia → Ogólne* to ten, do którego gra naprawdę zapisuje, i czy wciskasz klawisz zrzutu ekranu w grze.
- **Koledzy nie mogą dołączyć do mojego pokoju.** Muszą mieć dostęp do twojego PC. Przy otwieraniu pokoju użyj adresu z VPN typu mesh (Tailscale, ZeroTier) albo otwórz port drużyny na routerze.
- **Nadal nie działa?** Zapytaj na [Discordzie](https://discord.gg/H4pRxGBGDb). W *Ustawienia → Dziennik* jest przycisk *Kopiuj*, a wklejenie tego dziennika bardzo pomaga.

## Jak to działa

Rastro czyta dwa rodzaje plików, które Escape from Tarkov i tak zapisuje na twoim PC. Nic więcej nie potrzebuje.

| Co czyta | Gdzie | Co z tego wyciąga |
|---|---|---|
| **Nazwy twoich zrzutów ekranu** | `Documents\Escape from Tarkov\Screenshots` | Twoją pozycję i kierunek, w którym patrzysz. Tarkov zapisuje twoje współrzędne w nazwie każdego zrzutu. Rastro czyta nazwę, nigdy obraz, i nigdy nie przenosi, nie edytuje ani nie usuwa twoich zrzutów. |
| **Logi gry** (opcjonalne, wyłączone, dopóki ich nie włączysz) | `Battlestate Games\Escape from Tarkov\Logs` | Która mapa się wczytuje, kiedy zaczyna się rajd i czy grasz w PvP czy PvE. Dzięki temu Rastro sam przełącza się na właściwą mapę, zaczyna nowy ślad w każdym rajdzie i pokazuje pozostały czas rajdu. Zapisywane są tylko nazwa mapy i tryb gry. Czyta też wiadomości handlarzy, by odhaczać zadania, które przyjmiesz, oddasz lub oblejesz. |

W praktyce: wciskasz w grze klawisz zrzutu ekranu, a sekundę później twoja strzałka przesuwa się na mapie.

Czego Rastro **nie** robi: nie czyta pamięci gry, niczego nie wstrzykuje, nie podpina się pod proces gry, nie przechwytuje ruchu sieciowego, nie rysuje niczego na grze ani nie naciska za ciebie klawiszy. Działa jako osobny program i patrzy tylko na pliki na dysku.

## Budowanie ze źródeł

Potrzebujesz tego tylko, jeśli chcesz zmienić kod albo samodzielnie zbudować `.exe`. Będziesz potrzebować [Rusta](https://rustup.rs/) 1.85 lub nowszego oraz [Node.js](https://nodejs.org/) LTS (na Linuksie także `libwebkit2gtk-4.1-dev` i `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat na Windowsie
```

Wynikiem jest jeden plik wykonywalny w `target/release/`. Konfiguracja środowiska programistycznego, struktura projektu, zasady kodu i dodawanie tłumaczeń są opisane w **[CONTRIBUTING.md](../../CONTRIBUTING.md)** (po angielsku).
