# Translations

New feature in Czkawka 4.0 is ability to translate interface of GTK app.

App use Fluent localization system - https://projectfluent.org/

Main/Default language is English, but also Polish is officially supported.

## How to translate Czkawka?

Base translatable strings are placed under `i18n/en/czkawka_gui.ftl` file.  
`czkawka_core.ftl` is just a symlink which point to file from above(I had problems with splitting string into two separate crates)

Since such strings are heavily integrated into build system, so to check status of translation it is required to recompile Czkawka.

`czkawka_gui.ftl` file contains lines in this format:  
`id_of_message` = `translated_string`  
e.g.  
```
upper_manual_add_included_button_tooltip = Allows to add directory name to search by hand
```

to create new folder with translations, it is required to create copy of `i18n/en` folder and give it name from [ISO 639-1 code standard](https://www.loc.gov/standards/iso639-2/php/code_list.php) - e.g. pl, en, ru, fr etc. 

Next only translated strings needs to be changed

## Testing translation
### Replacing en folder
The simplest method is to remove `en` folder and replace it with needed one.  
Next Czkawka needs to be compiled and run.  

### Adding new translation
Recommended way to test translation, is to add it directly to Czkawka source code.

After creating proper and well named folder and translating string inside it, to be able to be able to choose this language file it is required to modify `czkawka_gui/src/language_functions.rs` file.

```rust
pub const LANGUAGES_ALL: [Language; 2] = [
    Language {
        combo_box_text: "English (en)",
        short_text: "en",
    },
    Language {
        combo_box_text: "Polski (pl)",
        short_text: "pl",
    },
];
```

The only thing which is required to change is `LANGUAGES_ALL` constant.

Number of items must be changed `[Language; 2]` -> `[Language; 3]`.

Next new record must be added to array.  
`combo_box_text` must contains translated language name(so `Polski` is used instead `Polish`), to help find people to find their native(or not) language.  
`short_text` is `ISO 639-1` code which need to match with county code and name of folder inside `i18n`.
```
    Language {
        combo_box_text: "Polski (pl)",
        short_text: "pl",
    },
```





