
function getCookie(cname) {
  let name = cname + "=";
  let decodedCookie = decodeURIComponent(document.cookie);
  let ca = decodedCookie.split(';');
  for(let i = 0; i <ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) == ' ') {
      c = c.substring(1);
    }
    if (c.indexOf(name) == 0) {
      return c.substring(name.length, c.length);
    }
  }
  return "";
}

var delete_cookie = function(name) {
    document.cookie = name + '=;expires=Thu, 01 Jan 1970 00:00:01 GMT;';
};

function logout() {
    delete_cookie("username");
    delete_cookie("id_key");
}

function create_activity(activity, value) {
    var div = document.createElement("div");
    div.classList.add("info_box2");
    div.style.width = "85%";
    //div.style.backgroundColor = "#FAFAFA";
    div.style.height = "60px";
    var activity_info_user = document.createElement("div");
    switch (activity.activity_type) {
        case 'Bike':
            activity_info_user.innerHTML = '<i class="fa fa-bicycle" style="font-size:30px;"></i>';
            break;
        case 'Run':
            activity_info_user.innerHTML = '<i class="fas fa-running" style="font-size:30px;"></i>&nbsp;&nbsp;&nbsp;';
            break;
        case 'Swim':
            activity_info_user.innerHTML = '<i class="fa fa-person-swimming" style="font-size:30px;"></i>&nbsp;';
            break;
        case 'Concert':
            activity_info_user.innerHTML = '<i class="fa fa-music" style="font-size:30px;"></i>&nbsp;';
            break;
        case 'Movie':
            activity_info_user.innerHTML = '<i class="fas fa-film" style="font-size:30px;"></i>&nbsp;';
            break;
        case 'Convention':
            activity_info_user.innerHTML = '<i class="fa fa-group" style="font-size:30px;"></i>&nbsp;';
            break;
    }

    activity_info_user.innerHTML += '&emsp;&emsp;&emsp;<b><a>' + activity.date.substring(0, 10) + "&emsp;" + activity.date.substring(11, 17) + '</a></b>'
    activity_info_user.innerHTML += "&emsp;&emsp;<a> created by </a>" + "<b>" + activity.username + "</b>";
    if(activity.username != getCookie("username")) {
        var join_form = document.createElement('form');
        join_form.className = "join_form";
        join_form.id = "form" + value;
        var join_button = document.createElement('button');
        activity_info_user.innerHTML += "&emsp;&emsp;&emsp;";
        join_button.className="button-23";
        join_button.innerHTML = "JOIN";
        join_button.id = activity.id;
        join_button.type = "submit";

        var activity_id = document.createElement('input');
        activity_id.type = "hidden";
        activity_id.name = "id";
        activity_id.value = activity.id;

        var activity_user = document.createElement('input');
        activity_user.type = "hidden";
        activity_user.name = "user";
        activity_user.value = getCookie("username");

        var activity_key = document.createElement('input');
        activity_key.type = "hidden";
        activity_key.name = "key";
        activity_key.value = getCookie("id_key");

        join_form.appendChild(join_button);
        join_form.appendChild(activity_id);
        join_form.appendChild(activity_user);
        join_form.appendChild(activity_key);
        activity_info_user.appendChild(join_form);
        activity_info_user.innerHTML += "<br><br>";
    }
    else activity_info_user.innerHTML += "<br><br>";
    div.appendChild(activity_info_user);

    activity_joined_by = document.createElement("div");
    if(activity.joined.length > 0) {
        activity_joined_by.innerHTML = "<a><b>JOINED BY: </b></a>";
        for(var i = 0; i < activity.joined.length; i++) {
            if(i != 0) activity_joined_by.innerHTML += ", "
            activity_joined_by.innerHTML += activity.joined[i];
        }
    }
    else activity_joined_by.innerHTML = "<a><b>NO ONE JOINED YET</b></a>";
    div.appendChild(activity_joined_by);

    return div;
}

function formSubmitJoin(event) {
  var url = "/join_activity";
  var request = new XMLHttpRequest();
  request.open('POST', url, true);
  var aux = 0;
  request.onreadystatechange = function() {
       if (this.readyState == 4 && this.status == 200) {
           update_activities_shown();
      }
      else if(this.status == 409) {
          if(aux == 0) alert("Need to first sign in!.");
          aux++;
          return;
      }
   };

  request.send(new FormData(event.target));
  event.preventDefault();
}

function attachFormSubmitEventJoin(formId){
  document.getElementById(formId).addEventListener("submit", formSubmitJoin);
}

function show_activites(activities) {
    var activities_div = document.getElementById("activities_div");
    activities_div.innerHTML = '<h1 style="text-align: center;">ACTIVITIES BOARD</h1><br>';
    for(var i = activities.length-1; i >= 0; i--) {
        if(document.getElementById(activities[i].activity_type.toLowerCase() + "_cbx").checked) {
            activities_div.appendChild(create_activity(activities[i], i))
            if(activities[i].username != getCookie("username")) attachFormSubmitEventJoin("form" + i);
        }
    }
}

function join_activity(elem) {
    fetch("/join_activity", {
        method: "POST",
        body: JSON.stringify({"id": elem.id})
    }).then((response) => {
        if (response.ok) {
            console.log("funciona");
        }
        else if(response.status == 409) alert("error joining activity.");
    });
}

function update_activities_shown() {
    fetch("/get_activities", {
        method: "GET",
    }).then((response) => {
        if (response.ok) {
            response.json().then(function(result) {
                show_activites(result);
            })
        }
        else if(response.status == 409) alert("error fetching activities.");
    });
}

function formSubmitCreate(event) {
  var url = "/create_activity";
  var request = new XMLHttpRequest();
  var aux = 0;
  request.onreadystatechange = function() {
     if (this.readyState == 4 && this.status == 200) {
         update_activities_shown();
    }
    else if(this.status == 422) {
        if(aux == 0) alert("Need to first sign in!.");
        aux++;
        return;
    }
 };

  request.open('POST', url, true);
  request.onload = function() {
    update_activities_shown();
  };

  request.send(new FormData(event.target));
  event.preventDefault();
}

function attachFormSubmitEventCreate(formId){
  document.getElementById(formId).addEventListener("submit", formSubmitCreate);
}

function init() {
    document.getElementById("id_key").value = getCookie("id_key")
    update_activities_shown();
    attachFormSubmitEventCreate("submit_activity")
    attachFormSubmitEventCreate("filter_activities")
    if("" != getCookie("username")) document.getElementById("logged_in_as").innerHTML = "LOGGED IN AS " + getCookie("username");
}

init();
