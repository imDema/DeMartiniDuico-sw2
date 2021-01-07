import yaml
import re
from sys import argv

template = r"""

\begin{tabu} to \textwidth {|X|X[4]|} \hline
  \textbf{GET}  &                       \\\hline
  Description   &   \\\hline
  Parameters    & \begin{tabu}{X|X|X[3]}
    \textbf{Name} & \textbf{Type} & \textbf{description} \\\hline
                 &         & 
  \end{tabu}  \\\hline
  Responses     & \begin{description}
    \tightlist
  \end{description} \\\hline
  Authorization &                    \\\hline
\end{tabu}
"""
def parse_schema(s):
    schema = parse_schema_sub(s)
    return f"\\texttt{{{schema}}}"
def parse_schema_sub(s):
    if "$ref" in s:
        ref = s["$ref"]
        return ref.split('/')[-1]
    type = s["type"]
    if type == "array":
        inner = parse_schema_sub(s["items"])
        return f"Array<{inner}>"
    elif type == "object":
        out = "\\{"
        for k, v in s["properties"].items():
            t = parse_schema_sub(v)
            out += f"{k}: {t}, "
        return out[:-1] + "\\}"
    else:
        return type

def extract_tag(k):
    k = k[1]
    if "get" in k:
        return k["get"]["tags"][0]
    elif "post" in k:
        return k["post"]["tags"][0]

def endpoints(y):
    paths = y["paths"]
    out = ""
    for path, p in sorted(paths.items(), key=extract_tag):
        out += "\\begin{table}[H]\n\\tabulinesep=4pt\everyrow{\\tabucline[0.5pt]-}\n"
        out += "\\begin{tabu} to \\textwidth {@{}|X|X[5]|} \\hline\n"
        if "get" in p:
            method = "GET"
            p = p["get"]
        elif "post" in p:
            method = "POST"
            p = p["post"]
        else:
            raise ()
        path = re.sub(r"\{", r"\{", re.sub(r"\}", r"\}", path))
        out += f"\\textbf{{{method}}}  & \\texttt{{{path}}} \\\\\n"
        desc = p["summary"]
        out += f"Description   & {desc}  \\\\\n"
        if "parameters" in p:
            out += "Parameters    & \\everyrow{}\\begin{tabu}{X|X}\n"
            out += "\\textbf{Name} & \\textbf{Type} \\\\\n"
            for par in p["parameters"]:
                name = par["name"]
                type = parse_schema(par["schema"])
                out += f"\hline {name} & {type} \\\\\n"

            out += "\\end{tabu}\\everyrow{\\tabucline[0.5pt]-}\\\\\n"
        if "requestBody" in p:
            type = parse_schema(p["requestBody"]["content"]["application/json"]["schema"])
            out += f"Body & {type} \\\\\n"
                
        out += "Responses     & \\everyrow{}\\begin{tabu}{X[0.5]|X[3]|X[2]} \n"
        out += "\\textbf{Code} & \\textbf{Description} & \\textbf{Body} \\\\\n"
        for code, resp in p["responses"].items():
            desc = resp["description"]
            out += f"\hline \\textbf{{{code}}} & {desc} &"
            if "content" in resp:
                type = parse_schema(resp["content"]["application/json"]["schema"])
                out += type
            out += "\\\\\n"
        out += "\\end{tabu}\\everyrow{\\tabucline[0.5pt]-} \\\\\n"

        out += "\\end{tabu}\n"
        out += "\\end{table}\n"
    return out

def summary(y):
    out = """
\\begin{tabularx}{\\textwidth}{|p{0.08\\textwidth}|l|X|} \\hline
\\textbf{Class} & \\textbf{HTTP request} & \\textbf{Description} \\\\\\hline\\endhead
"""

    paths = y["paths"]
    for path, p in sorted(paths.items(), key=extract_tag):

        if "get" in p:
            method = "GET"
            p = p["get"]
        elif "post" in p:
            method = "POST"
            p = p["post"]
        else:
            raise ()

        cl = p["tags"][0]
        path = re.sub(r"\{", r"\{", re.sub(r"\}", r"\}", path))
        req = f"\\makecell{{\\textbf{{{method}}} \\\\ \\texttt{{{path}}}}}"
        desc = p["summary"]

        out += f"{cl} & {req}  & {desc} \\\\\\hline\n"

    out += "\\end{tabularx}\n\n"
    return out

def models(y):
    out = ""
    comp = y["components"]["schemas"]
    for name, content in sorted(comp.items(), key=lambda k: k[0]):
        if content["type"] == "object" and not "properties" in content:
            continue
        if content["type"] == "object":
            out += f"""
    \\begin{{table}}[H]
    \\centering
    \\textbf{{{name}}}\\\\
    \\everyrow{{\\tabucline[0.5pt]-}}
    \\begin{{tabu}} to 0.55\\textwidth {{|X|X|}} \\hline
    Field & Type \\\\
    """

            props = content["properties"]

            for key, value in props.items():
                if value["type"] == "object":
                    type = key
                else:
                    type = parse_schema(value)
                
                out += f"{key} & {type} \\\\\n"
            out += "\\end{tabu}\n"
            out += "\\end{table}\n\n"
        else:
            type = parse_schema(content)
            out += f"\\begin{{center}}\\textbf{{{name}}}: {type}\\end{{center}}\n\n"
    return out


infile = argv[1]

# print(infile)

with open(infile, "r") as infile:
    y = yaml.full_load(infile)

out = "\\subsubsection{API}\n\n" + summary(y) +\
    "\\subsubsection{Endpoints}\n\n" + endpoints(y) +\
    "\\subsubsection{Models}\n\n" + models(y)

out = re.sub(r"\$", r"\\$", out)
out = re.sub(r"#", r"\#", out)
out = re.sub(r"_", r"\_", out)


print(out)