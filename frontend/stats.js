// import { ADDRESS } from './env.js'
function open_statistic(statistic_id, statistic_name) {
    return async function () {
        console.log(statistic_id);
        const response = await fetch(
            `${ADDRESS}/load/stat/${statistic_id}`,
            {
                method: 'GET',
            }
        );
        const data = await response.json(); // Extracting data as a JSON Object from the response
        console.log(data);
        sessionStorage.setItem("objects", JSON.stringify(data));
        sessionStorage.setItem("curr_stat", JSON.stringify({ name: statistic_name, id: statistic_id }));
        window.location.href = "statistic.html";
    }
}
async function update_storage(user_id) {

    const response = await fetch(
        `${ADDRESS}/stats/${user_id}`,
        {
            method: 'GET',
        }
    );
    const data = await response.json(); // Extracting data as a JSON Object from the response
    sessionStorage.setItem("stat", JSON.stringify(data));

}
function delete_statistic(statistic_id) {
    return async function () {
        var user_id1 = sessionStorage.getItem("userId");
        user_id1 = JSON.parse(user_id1);
        user_id1 = parseInt(user_id1);
        console.log(statistic_id);
        const response = await fetch(
            `${ADDRESS}/stats/delete/`,
            {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: statistic_id

            }
        )
        const data = await response.json(); // Extracting data as a JSON Object from the response
        console.log(data);
        const result = await update_storage(user_id1);
        load();
    }

}
async function create_statistic(submit_event) {
    submit_event.preventDefault();
    let statistic_name = document.getElementById('statName').value;
    document.getElementById('statName').value = "";
    var user_id1 = sessionStorage.getItem("userId");
    user_id1 = JSON.parse(user_id1);
    user_id1 = parseInt(user_id1);

    const response = await fetch(
        `${ADDRESS}/stats/add/`,
        {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name: statistic_name, user_id: user_id1 })
        }
    );
    const data = await response.json(); // Extracting data as a JSON Object from the response
    console.log(data);
    const result = await update_storage(user_id1);
    load();
}
function rename_statistic(statistic_id, statistic_name) {
    return async function () {
        var user_id1 = sessionStorage.getItem("userId");
        user_id1 = JSON.parse(user_id1);
        user_id1 = parseInt(user_id1);
        let statistic_new_name = prompt("Enter new name", statistic_name);
        if (statistic_new_name == null || statistic_new_name == "") {
            return;
        }
        const response = await fetch(
            `${ADDRESS}/stats/rename/`,
            {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ id: statistic_id, name: statistic_new_name })
            }
        );
        const data = await response.json();
        console.log(data);
        const result = await update_storage(user_id1);
        load();
    }
}
document.getElementById('create').addEventListener('submit', create_statistic);

function load() {
    document.getElementById("statistics").innerHTML = "";
    var stat = sessionStorage.getItem("stat");
    stat = JSON.parse(stat);
    for (var i in stat) {
        const node = document.createTextNode(stat[i].name);

        var separate = document.createElement("div");
        var neo_link = document.createElement('button');
        var delete_button = document.createElement('button');

        delete_button.className = "delete-class";
        delete_button.innerHTML = "X";
        delete_button.onclick = delete_statistic(stat[i].id);

        var rename_button = document.createElement('button');
        var icon = document.createElement('i');

        icon.className = "fa-solid fa-pen-to-square";
        rename_button.onclick = rename_statistic(stat[i].id, stat[i].name);
        rename_button.className = "rename-btn";
        rename_button.appendChild(icon);

        var text = document.createTextNode(stat[i].name);
        var new_line = document.createElement("br");
        
        neo_link.appendChild(text);
        neo_link.appendChild(new_line);
        neo_link.onclick = open_statistic(stat[i].id, stat[i].name);
        neo_link.className = "stat-text";
        
        separate.appendChild(neo_link);
        separate.appendChild(rename_button);
        separate.appendChild(delete_button);
        
        document.getElementById("statistics").appendChild(separate);
    }
}

load();