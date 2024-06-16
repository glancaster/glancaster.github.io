document.addEventListener("DOMContentLoaded", function () {
  // Execute JavaScript code after the DOM has been fully loaded
  var contentDiv = document.getElementById("topnav");
  // Create navigation elements
  var header = document.createElement("header");
  var nav = document.createElement("nav");
  var ul = document.createElement("ul");

  // Define navigation items
  var navItems = ["Home", "About", "Projects"];

  navItems.forEach(function (item) {
    var li = document.createElement("li");
    var a = document.createElement("a");
    a.textContent = item;
    a.href = "#"; // Set href to '#' for demonstration purposes
    li.appendChild(a);
    ul.appendChild(li);

    li.addEventListener("mouseenter", function () {
      li.style.backgroundColor = "darkgray";
      a.style.color = "lightgray";
    });
    li.addEventListener("mouseleave", function () {
      li.style.backgroundColor = "transparent";
      a.style.color = "#333";
    });
  });

  nav.appendChild(ul);
  header.appendChild(nav);
  contentDiv.appendChild(header);

  // Add styling to the navigation bar
  contentDiv.style.backgroundColor = "lightgray";
  contentDiv.style.padding = "1rem";
  contentDiv.style.borderRadius = "1rem";

  header.style.color = "#fff";
  header.style.padding = "1rem";

  nav.style.display = "flex";
  nav.style.justifyContent = "center";

  ul.style.listStyleType = "none";
  ul.style.margin = "0";
  ul.style.padding = "0";

  ul.querySelectorAll("li").forEach(function (li) {
    li.style.padding = "1.5rem 2rem 1rem";
    li.style.borderRadius = "1rem";
    li.style.display = "inline";
    li.style.marginRight = "2rem";
  });

  ul.querySelectorAll("a").forEach(function (a) {
    a.style.color = "#333";
    a.style.textDecoration = "none";
    a.style.fontSize = "1.5rem";
  });
});
