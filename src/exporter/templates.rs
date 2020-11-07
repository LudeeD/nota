pub const skeleton : &str =
"<!DOCTYPE HTML>
<html>
<head>
    <meta charset=\"utf-8\" />
    <title>App</title>
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
    <script charset=\"utf-8\" src=\"js/app.js\"></script>
    <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/water.css@2/out/water.css\">
</head>
<body>
    {{#if entry}}
    <form action=\"index.html\">
        <input style=\"float: right;\" type=\"submit\" value=\"Back to Index 🏡\" />
    </form>
    {{/if}}
    {{{body}}}
    {{#if entry}}
    <hr>
    <a href=\"./index.html\" style=\"float: left;\">⬅️ Back to index</a>
    <a href=\"#top\" style=\"float: right;\">To the top ⬆️</a>
    <br/>
    {{/if}}
<div/>
</body>
</html>";