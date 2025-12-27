# AI Translation Script for FTL Files

Skrypt do automatycznego tłumaczenia plików FTL (Fluent) używając offline modelu AI (Ollama).

## Wymagania

1. **Ollama** - zainstaluj z https://ollama.ai/
2. **Python 3.8+**
3. **Python biblioteki** - zainstaluj używając:
   ```bash
   pip install -r requirements.txt
   ```

## Instalacja modelu Ollama

Po zainstalowaniu Ollama, pobierz model tłumaczeniowy:

```bash
# Zalecany model (7B parametrów, dobry balans jakości i szybkości)
ollama pull qwen2.5:7b

# Lub inne modele:
ollama pull llama3.1:8b
ollama pull mistral:7b
```

## Użycie

### Najszybszy sposób (z justfile)

Jeśli pracujesz w głównym repozytorium Czkawka, możesz użyć reguł z justfile:

**Instalacja zależności (tylko raz):**
```bash
just prepare_translations_deps
```

**Tłumaczenie wszystkich projektów:**
```bash
just translate
```

### Podstawowe użycie

Tłumaczenie wszystkich języków w folderze i18n:

```bash
python3 misc/ai_translate/translate.py czkawka_gui/i18n
```

Dla krokiet:
```bash
python3 misc/ai_translate/translate.py krokiet/i18n
```

### Zaawansowane opcje

**Użycie innego modelu:**
```bash
python3 misc/ai_translate/translate.py czkawka_gui/i18n --model llama3.1:8b
```

**Dry run (podgląd bez zmian):**
```bash
python3 misc/ai_translate/translate.py czkawka_gui/i18n --dry-run
```

**Tłumaczenie tylko wybranych języków:**
```bash
python3 misc/ai_translate/translate.py czkawka_gui/i18n --languages pl de fr
```

## Jak to działa

1. Skrypt wczytuje plik angielski (en/nazwa.ftl) jako bazowy
2. Parsuje wszystkie klucze i wartości (obsługuje wartości wielolinijkowe)
3. Dla każdego języka:
   - Wczytuje istniejący plik tłumaczenia
   - Znajduje brakujące klucze
   - Znajduje klucze z nieprzetłumaczonymi wartościami (identyczne z angielskim)
   - Tłumaczy wartości używając modelu AI
   - Zapisuje aktualizacje zachowując strukturę i komentarze

## Przykładowe wyjście

```
🌍 Processing i18n folder: /path/to/czkawka_gui/i18n
📄 Base file: czkawka_gui.ftl
📊 Found 320 entries in base file

🔤 Processing language: pl (Polish)
    ➕ Missing key: new_feature_button
       Translating: Click here for new feature...
    🔄 Untranslated key (same as English): duplicate_mode_name
       Translating: Name...
    ✅ Updated czkawka_gui.ftl with 2 translations

🔤 Processing language: de (German)
    ➕ Missing key: new_feature_button
       Translating: Click here for new feature...
    ✅ Updated czkawka_gui.ftl with 1 translations

✨ Complete! Total translations: 3
```

## Struktura plików FTL

Skrypt rozpoznaje pliki FTL w formacie:

```fluent
# Komentarz
key = wartość

multiline_key =
    Pierwsza linia
    Druga linia
    Trzecia linia

another_key = Prosta wartość
```

## Wsparcie języków

Skrypt obsługuje następujące języki:
- ar (Arabic)
- bg (Bulgarian)
- cs (Czech)
- de (German)
- el (Greek)
- es-ES (Spanish)
- fa (Persian)
- fr (French)
- it (Italian)
- ja (Japanese)
- ko (Korean)
- nl (Dutch)
- no (Norwegian)
- pl (Polish)
- pt-BR (Brazilian Portuguese)
- pt-PT (Portuguese)
- ro (Romanian)
- ru (Russian)
- sv-SE (Swedish)
- tr (Turkish)
- uk (Ukrainian)
- zh-CN (Simplified Chinese)
- zh-TW (Traditional Chinese)

## Rozwiązywanie problemów

### Ollama nie działa

Sprawdź czy Ollama jest uruchomiona:
```bash
ollama list
```

Jeśli nie działa, uruchom:
```bash
ollama serve
```

### Model nie jest pobrany

Pobierz model:
```bash
ollama pull qwen2.5:7b
```

### Błędy tłumaczenia

Jeśli tłumaczenia są niskiej jakości, wypróbuj inny model lub dostosuj prompt w kodzie.

## Licencja

Ten skrypt jest częścią projektu Czkawka.

