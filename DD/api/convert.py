import re
from os import listdir
from os.path import isfile, join
def append(outfile, path):
    with open(path, "r") as infile:
        outfile.write(infile.read())

def read(path):
    with open(path, "r") as infile:
        return infile.read()

def replace(content, rules):
    ret = content
    for k, v in rules.items():
        ret = re.sub(k, v, ret)
    return ret

with open("api.md", "w") as outfile:

    # Readme
    readme = replace(read("./gen/README.md"),{
        r"(.*\|).*\|(.*\|.*)": r"\1\2",
        r"<a .+</a>": r"",
        r"- \*\*Location\*\*: ": r"",
        r"All URIs.+": r"",
        r"^# Docu.+": r"",
        r"\n\n\n": r"\n\n",})
    outfile.write(readme)

    # Apis
    path = "./gen/Apis"
    apifiles = [join(path,f) for f in listdir(path) if isfile(join(path, f))]
    for api in apifiles:
        content = replace(read(api),{
            r"": r"",
            r"^# \w+Api[^<]+": r"",
            r"<a .+</a>": r"",
            r"All URIs.+": r"",
            r"(.*\|.*\|.*)\|.*": r"\1",
            r"### HTTP request headers[^#]+": r"",
            r"### Return type\n\nnull[^#]+": r"",
            r"### Parameters\nThis endpoint does not need any parameter[^#]": r"",
            r"\n\n\n|\n\n\n\n|\n\n\n\n\n": r"\n\n",
            r"(^|\n)#": r"\1####"
            })
        outfile.write(content)

    # Models
    path = "./gen/Models"
    models = [join(path,f) for f in listdir(path) if isfile(join(path, f))]
    for mod in models:
        content = replace(read(mod),{
            r"<a .+</a>": r"",
            r"^## Prop.+": r"",
            r"(.*\|.*)\|.*\|.*": r"\1",
            r"\[\[Ba.+": r"",
            r"\n\n\n": r"\n\n",
            r"(^|\n)#": r"\1####"
            })
        outfile.write(content)