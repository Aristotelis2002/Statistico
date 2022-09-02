var curr_stat_id;
document.getElementById('add').addEventListener('submit',add_object);
document.getElementById('visualize').onclick = get_chart


function get_chart(){
    document.getElementById("chart").innerHTML = "";
    var img = document.createElement("img");
    img.src=`${ADDRESS}/visualize/${curr_stat_id}?${(new Date()).getTime()}`;
    document.getElementById("chart").appendChild(img);

    
}
async function add_object(submit_event){
    submit_event.preventDefault();
    let object_name = document.getElementById('objName').value;
    let counter_value = document.getElementById('counter').value;
    document.getElementById('objName').value = "";
    document.getElementById('counter').value = "0";

    counter_value = parseInt(counter_value);
    const response = await fetch(
		`${ADDRESS}/object/add/`,
		{
			method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({name: object_name,
                 statistic_id: curr_stat_id,
                 counter: counter_value})
		}
	);
	const data = await response.json(); 
    console.log(data);
    const result = await update_storage_objects();
    load();
}
async function update_storage_objects(){
    const response = await fetch(
		`${ADDRESS}/load/stat/${curr_stat_id}`,
		{
			method: 'GET',
		}
	);
	const data = await response.json(); 
    sessionStorage.setItem("objects",JSON.stringify(data));
}
function delete_object(object_id){
    return async function(){
        const response = await fetch(
            `${ADDRESS}/object/delete/`,
		{
			method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: object_id
        
		}
        )
        const data = await response.json();
        console.log(data);
        const result = await update_storage_objects(curr_stat_id);
        load();
    }
    
}
function rename_object(object_id, object_name){
    return async function(){
        let object_new_name = prompt("Enter new name", object_name);
        if (object_new_name == null || object_new_name == "") {
            return;
        }
        const response = await fetch(
            `${ADDRESS}/object/rename/`,
            {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ id: object_id, name: object_new_name })
            }
        );
        const data = await response.json();
        console.log(data);
        const result = await update_storage_objects(curr_stat_id);
        load();
    }
}
function crement_object(object_id, diff){
    return async function(){
        let counter_value = document.getElementById(object_id).value;
        counter_value = parseInt(counter_value);
        counter_value = counter_value + diff;
        const response = await fetch(
            `${ADDRESS}/object/counter/update/`,
		{
			method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({id: object_id,
                counter: counter_value})        
		}
        )
        const data = await response.json();
        console.log(data);
        const result = await update_storage_objects(curr_stat_id);
        document.getElementById(object_id).value = counter_value;
    }
}
////TODO
//button rename za imeto
//handler za ruchno promenqne
//button visualize graph/statistic
function load(){
    document.getElementById("objects").innerHTML = "";
    document.getElementById("stat_name").innerHTML = "";
    var objects = sessionStorage.getItem("objects");
    objects = JSON.parse(objects);
    var statistic_info = sessionStorage.getItem("curr_stat");
    statistic_info = JSON.parse(statistic_info);
    curr_stat_id = statistic_info.id;
    var html_name = document.createElement('div'); //<--
    const node = document.createTextNode(statistic_info.name);
    html_name.appendChild(node);
    document.getElementById("stat_name").appendChild(html_name);
    for (i in objects){
        const node = document.createTextNode(objects[i].name);
        const white_space =document.createTextNode("\u00A0");
        var separate = document.createElement("div");
        var field = document.createElement("div");
        var counter = document.createElement("input");
        var button_incr = document.createElement("button");
        var button_decr = document.createElement("button");
        var button_del = document.createElement("button");
        var new_line = document.createElement("br");
        var rename_button = document.createElement('button');
        var icon = document.createElement('i');
        icon.className = "fa-solid fa-pen-to-square";
        rename_button.onclick = rename_object(objects[i].id, objects[i].name);
        rename_button.className = "rename-btn";
        rename_button.appendChild(icon);
        button_del.className = "delete-class";
        button_del.innerHTML = "X";
        button_del.onclick = delete_object(objects[i].id);
        button_incr.innerHTML="+";
        button_incr.className = "button-incr";
        button_incr.onclick = crement_object(objects[i].id, 1);
        button_decr.innerHTML="-";
        button_decr.className = "button-decr";
        button_decr.onclick = crement_object(objects[i].id, -1);
        counter.type = "text";
        counter.id = objects[i].id;
        counter.value = objects[i].counter;
        counter.className = "counter-class";
        field.appendChild(node);
        field.appendChild(white_space);
        field.appendChild(rename_button);
        field.appendChild(button_del);
        separate.appendChild(field);
        separate.appendChild(new_line);
        separate.appendChild(button_incr);
        separate.appendChild(counter);
        separate.appendChild(button_decr);
        separate.className = "separate-object";
        document.getElementById("objects").appendChild(separate);
    }
}
load();