<div align="center">
<h1>Paintshare</h1>
<hr>
<strong>A social media platform for sharing self drawed images</strong><br><br>
</div>
<div align="center">
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png" height="100" />
<img src="https://cdn.freebiesupply.com/logos/thumbs/2x/react-1-logo.png" height="100" />
</div>


# Project information

PaintShare is a social media platform for self drawed images. It was ment as a birthday gift for my little brother who wanted to have a platform to post all his images on.
The backend is built with rust, because it is very fast and can handle big workloads with low memory and COU consumption. The fronend is built with React, because it is an modern
multipurpose and fast web framework. The development process of this project is stopped.

# Updates

This project will not recive any updates at all. The project is working partly, but is not finished at all. Because my brother just wanted a local instance, there is no real need
for extra features that would be useful on a public instance. But because the market for such a webapp is too low and the development process would take too long for such less
earn from it, I stopped working on it. You can feel free to contribute to this project. I will accept all good pull_requests to this project. But it would be hard to validate 
them, because this repository does not have any CI/CD pipelines for validating the code-quality and other important things in the pull_request.

# Contribute

As I said, feel free to open pull_requests to this repo to make this project grow. Of course, I will review each pull_request on my own to check, if the code is working properly.
Because there are no CI/CD pipelines given, it is hard to check all the requirements, which could take a long time. Feel free to live your own code style as long as it is readable 
and your code is documented properly with a fluent english language. Even if there are no needs for unit testing, provide some test cases. This would make the work to review your 
pull_request easier and faster for me. Please also try to use given libaries and reduce duplications in your code. At all you can say, just keep the codebase clean and you can
add any feature you want, except spy features.

# Setup

There is no setup given in this project. You can use the public docker image hosted on the docker hub to run the project`s docker image. But it can happen, that the
server crashes often, because it is an development server and has not been tested with higher workloads and more than 10 clients at one tine. 
There is no other simple way to host the project by your own. But as I do not recommend to host this project at all, this should not be a big issue.

# Techstack

rust => 1.43<br>
actix-web => 3<br>
react => 17<br>
typescript => 4.2.4<br>
fontawsome => 1.2.35<br>
