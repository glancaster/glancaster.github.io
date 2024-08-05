import glob
from jinja2 import Template
import markdown
import os
###### Base ######

# Base HTML Template
baseTemplate = Template('''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Graham Lancaster | {{ title }}</title>
    <link rel="stylesheet" href="styles.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/p5.js/0.6.0/p5.js"></script>
    <script src="sources/mold.js"></script>
</head>
<body>
    <header>
        <ul>
            {% for header in headers -%}
                <a href="{{headers[header]}}.html">{{header}}</a> 
            {% endfor %}
        </ul>
    </header>
    <main>
        <div class="content">
        {{ content }}
        </div>
    </main>
    <footer>
        <ul>{{ footer }}</ul>
    </footer>
</body>
</html>
''')
homeTemplate = Template('''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Graham Lancaster | {{ title }}</title>
    <link rel="stylesheet" href="styles.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/p5.js/0.6.0/p5.js"></script>
    <script src="sources/mold.js"></script>
</head>
<body>
    <header>
        <ul>
            {% for header in headers -%}
                <a href="{{headers[header]}}.html">{{header}}</a> 
            {% endfor %}
        </ul>
    </header>
    <main>
        <div class="home">
        {{ content }}
        </div>
    </main>
    <footer>
        <ul>{{ footer }}</ul>
    </footer>
</body>
</html>
''')
itemTemplate = Template('''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Graham Lancaster | {{ title }}</title>
    <link rel="stylesheet" href="styles.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/p5.js/0.6.0/p5.js"></script>
    <script src="sources/mold.js"></script>
</head>
<body>
    <header>
        <ul>
            {% for header in headers -%}
                <a href="{{headers[header]}}.html">{{header}}</a> 
            {% endfor %}
        </ul>
    </header>
    <main>
        <div class="items">                
        <ul>
            {% for item in items -%}
                <li><a href="{{items[item].Ref}}">{{item}}</a>
                    <p>{{items[item].Description}}</p></li> 
            {% endfor %}
        </ul>
        </div>
    </main>
    <footer>
        <ul>{{ footer }}</ul>
    </footer>
</body>
</html>
''')
# Base CSS Content
baseCSS = '''
a {
  color: lightgray;
  text-decoration: rgb(94, 94, 94) underline 3px;
}
body {
  color: lightgray;
  font-family: monospace, sans-serif;
  margin: 0;
  line-height: 1.6;
  padding: 15px;
}
canvas {
  position: absolute;
  top: 0;
  left: 0;
  z-index: -1; /* Places the canvas behind all other content */
}
header,
footer,
.content {
  background-color: #333;
  color: lightgray;
  text-align: left;
  padding: 1em 0;
  border-radius: 25px;
}
.content{
  padding: 10px 20px;
  margin-top: 30px;
}
header ul,
footer ul {
  display: flex;
  justify-content: center;
  gap: 30px;
}
header a,
footer a {
  background: #333;
  padding: 10px 50px;
  border-radius: 25px;
  transition: background-color 0.3s ease;
}
header a:hover,
footer a:hover,
.items li:hover {
  color: lightgray;
  background-color: rgb(52, 79, 47); /* Darker shade on hover */
}
.items a:hover {
  color: lightgray;
}
.items {
  padding: 2em;
  text-align: center;
}
.items ul {
  display: flex;
  justify-content: center;
  gap: 20px;
  flex-wrap: wrap;
  flex-direction: row;
}
.items li {
  background: #333;
  padding: 10px;
  border-radius: 25px; /* Rounded corners */
  transition: background-color 0.3s ease;
  list-style: none;
}

footer {
  bottom: 0;
  position: fixed;
  width: 98%;
  margin-bottom: 20px;
}

/* Media Queries for Mobile Responsiveness */
@media (max-width: 768px) {
  body {
    padding: 10px;
  }

  header,
  footer {
    padding: 20px 0;
  }

  header ul,
  footer ul {
    gap: 10px;
    flex-wrap: wrap;
  }

  header a,
  footer a {
    padding: 10px 15px;
  }
}

'''

# Data to fill in the HTML template
baseData = {
    "title": "Insert Title",
    "headers": {
        "Graham Lancaster":"index",
        "Projects":"projects",
        "Libraries":"libraries",
        "About":"about"
    },
    "content": "Insert Content",
    "footer": "© 2024 Graham Lancaster"
}

def md2html(file):
    d,f = os.path.split(file)
    b,ext = os.path.splitext(f)

    with open(file, "r", encoding="utf-8") as mdfile:
        mdData = baseData.copy()
        mdData['content'] = markdown.markdown(mdfile.read())
        mdData['title'] = b
        htmlcontent = baseTemplate.render(mdData)

    
    with open(b + ".html", "w", encoding="utf-8") as htmlfile:
        htmlfile.write(htmlcontent)
    return b + ".html"


###### About ######
md2html("src/about.md")

###### Projects ######
projects = glob.glob("projects/*.md")
projectData = baseData.copy()
projectData['title'] = "projects" 
projectData['items'] = dict()

for project in projects:
    with open(project, 'r', encoding='utf-8') as projectfile:
        projectheader = next(projectfile).split(maxsplit=1)[1].strip()
        print(projectheader)
        projectdesc = list(projectfile)[3].strip()
        print(projectdesc)
        projectData["items"].update({projectheader:{"Description":projectdesc,
                                                       "Ref":md2html(project)}})       
                

projecthtml = itemTemplate.render(projectData)
with open("projects.html", "w", encoding="utf-8") as htmlfile:
    htmlfile.write(projecthtml)




###### Libraries ######
libs = glob.glob("libraries/*.md")
libData = baseData.copy()
libData['title'] = "libraries" 
libData['items'] = dict()
for lib in libs:
    with open(lib, 'r', encoding='utf-8') as libfile:
        libheader = next(libfile).split(maxsplit=1)[1].strip()
        print(libheader)
        libdesc = list(libfile)[3].strip()
        print(libdesc)
        libData["items"].update({libheader:{"Description":libdesc,
                                                       "Ref":md2html(lib)}})       

libhtml = itemTemplate.render(libData)
with open("libraries.html", "w", encoding="utf-8") as htmlfile:
    htmlfile.write(libhtml)



###### Home ######
""" Using Results of Projects and Libraries to build up the Home Page"""
with open("index.html", "w", encoding="utf-8") as html_file:
    mdData = baseData.copy()
    mdData['title'] = "home" 
    mdData['content'] = ""
    html_file.write(homeTemplate.render(mdData))



###### CSS ######
with open("styles.css", "w", encoding="utf-8") as css_file:
    css_file.write(baseCSS)





print("Website generated successfully.")
