translations = ["pl", "it", "de"] # en is missing here
base_translation = "en"

base_keywords = []
with open('i18n/' + base_translation + "/czkawka_gui.ftl", 'r') as file:
    base_translation_file_content = file.readlines()
    for line in base_translation_file_content:
        if line.find("=") != -1:
            first_split = line.split("=")[0].strip()
            # Debug check, check for usage of AAAAA in result to see which esults are unused
            # To check in what exactly places are usused, remove -c parameter
            # print("rg \"" + first_split + "\" czkawka_gui/src -c;echo \"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA   -      "+first_split+"\"")
            try:
                base_keywords.index(first_split)
                print("Duplicated member " + first_split +" in base translation")
            except:
                True # All good
            base_keywords.append(first_split)


for lang in translations:
    print("\nChecking " + lang + " language")
    lang_keywords = []
    with open('i18n/' + lang + "/czkawka_gui.ftl", 'r') as file:
        file_content = file.readlines()
        for line in file_content:
            if line.find("=") != -1:
                first_split = line.split("=")[0].strip()
                try:
                    lang_keywords.index(first_split)
                    print("Duplicated member " + first_split +" in " + lang + " translation")
                except:
                    True # All good
                lang_keywords.append(first_split)
        
    for keyword in base_keywords:
        try:
            lang_keywords.index(keyword)
        except:
            print("Missing keyword - " + keyword)

    for keyword in lang_keywords:
        try:
            base_keywords.index(keyword)
        except:
            print("Unused keyword - " + keyword)

