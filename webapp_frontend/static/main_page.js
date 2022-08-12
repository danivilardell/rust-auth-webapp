

function init() {
    const create_activity_form = document.getElementById('form');
    create_activity_form.addEventListener('submit', create_activity);
}

function create_activity(event) {
    event.preventDefault();
    const data = new FormData(event.target);

    const value = Object.fromEntries(data.entries());

    console.log({ value })
}

init();