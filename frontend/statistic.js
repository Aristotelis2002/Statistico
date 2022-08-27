var curr_stat_id;
document.getElementById('add').addEventListener('submit',add_object);
async function add_object(submit_event){
    submit_event.preventDefault();
    let object_name = document.getElementById('objName').value;
    let counter_value = document.getElementById('counter').value;
    document.getElementById('objName').value = "";
    document.getElementById('counter').value = "0";

    counter_value = parseInt(counter_value);
    const response = await fetch(
		`http://127.0.0.1:8080/object/add/`,
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
		`http://127.0.0.1:8080/load/stat/${curr_stat_id}`,
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
            `http://127.0.0.1:8080/object/delete/`,
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
function crement_object(object_id, diff){
    return async function(){
        let counter_value = document.getElementById(object_id).value;
        counter_value = parseInt(counter_value);
        counter_value = counter_value + diff;
        const response = await fetch(
            `http://127.0.0.1:8080/object/counter/update/`,
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
        //load();
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
        field.appendChild(button_del);
        // field.appendChild(new_line);
        // field.appendChild(new_line);
        separate.appendChild(field);
        // separate.appendChild(white_space);
        // separate.appendChild(button_del);
        separate.appendChild(new_line);
        separate.appendChild(button_incr);
        separate.appendChild(counter);
        separate.appendChild(button_decr);
        // separate.appendChild(new_line);
        separate.className = "separate-object";
        document.getElementById("objects").appendChild(separate);
       // document.getElementById("objects").appendChild(counter);
    }
}
load();