pub const entry : &str =
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
<div class=\"row\">
    <div>
        <h1> Title title </h1> 
        <p> Last Modified: akljdlakjdlakjd </p>
    </div>
    <a href=\"index.html\" style=\"float: right;\"><p> Last Modified: akljdlakjdlakjd </p></a>
</div>
<hr>

    {{{body}}}
    <hr>
    <a href=\"./index.html\" style=\"float: left;\">⬅️ Back to index</a>
    <a href=\"#top\" style=\"float: right;\">To the top ⬆️</a>
    <br/>
<div/>
</body>
</html>";

pub const index : &str =
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
    <br/>
    <h1> Index List <h1/>
    <hr>
    <ul>
        {{#each nav}}
        <a href=\"{{link}}\"><li>{{title}}</li></a>
        {{/each}}
    </ul>  
<div/>
</body>
</html>";