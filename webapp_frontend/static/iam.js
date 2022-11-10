

function init() {
    let sign_up_button = document.getElementById("button_sign_up");
    let sign_in_button = document.getElementById("button_sign_in");
    sign_up_button.addEventListener("click", sign_up);
    sign_in_button.addEventListener("click", sign_in);
}

function sign_up() {
    username = document.getElementById("username").value;
    password = document.getElementById("password").value;

    fetch("/sign_up", {
        method: "POST",
        body: new URLSearchParams({ username, password }),
    }).then((response) => {
        if (response.ok) alert("Successfull sign up.");
        else if(response.status == 409) alert("Username already exists.");
    });
}

function sign_in() {
    username = document.getElementById("username").value;
    password = document.getElementById("password").value;

    fetch("/sign_in", {
        method: "POST",
        body: new URLSearchParams({ username, password }),
    }).then((response) => {
        if (response.ok) {
            response.json().then(function(result) {
                window.location.href = "/main_page.html";
                document.cookie = "id_key=" + result.id;
            })
        }
        else if(response.status == 409) alert("Wrong username or password.");
    });
}

init();
