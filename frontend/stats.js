
function open_statistic(statistic_id){
    return async function(){
        console.log(statistic_id); 
    const response = await fetch(
		`http://127.0.0.1:8080/load/stat/${statistic_id}`,
		{
			method: 'GET',
		}
	);
	const data = await response.json(); // Extracting data as a JSON Object from the response
    console.log(data);
    sessionStorage.setItem("objects",JSON.stringify(data));
    window.location.href = "statistic.html";
    }
}
async function update_storage(user_id){
    
    const response = await fetch(
		`http://127.0.0.1:8080/stats/${user_id}`,
		{
			method: 'GET',
		}
	);
	const data = await response.json(); // Extracting data as a JSON Object from the response
   // console.log(data);
    sessionStorage.setItem("stat",JSON.stringify(data));

}
async function create_statistic(submit_event) {
    submit_event.preventDefault();
    let statistic_name = document.getElementById('statName').value;
    var user_id1 = sessionStorage.getItem("userId");
    user_id1 = JSON.parse(user_id1);
    user_id1 = parseInt(user_id1);
    
    const response = await fetch(
		`http://127.0.0.1:8080/stats/add/`,
		{
			method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({name: statistic_name, user_id: user_id1})
		}
	);
	const data = await response.json(); // Extracting data as a JSON Object from the response
    console.log(data);
    //sessionStorage.setItem("stat",JSON.stringify(data));
   const result = await update_storage(user_id1);
    reload();
}
document.getElementById('create').addEventListener('submit',create_statistic);
//podai imeto na statistikata kum href
//Button suzdai nova statistika
//button iztrii stara statistika
//button rename statistika
function load(){
document.getElementById("statistics").innerHTML = "";
var stat = sessionStorage.getItem("stat");
stat = JSON.parse(stat);
for(var i in stat){
    const node = document.createTextNode(stat[i].name);
          
    var neo_link = document.createElement('button'); 
      
    var text = document.createTextNode(stat[i].name);
    var new_line = document.createElement("br");  
    neo_link.appendChild(text);
    neo_link.appendChild(new_line);
    neo_link.onclick = open_statistic(stat[i].id); 
    document.getElementById("statistics").appendChild(neo_link);
    console.log(i);
}
}
function reload(){
    document.getElementById("statistics").innerHTML = "";
    var stat = sessionStorage.getItem("stat");
    stat = JSON.parse(stat);
    for(var i in stat){
        const node = document.createTextNode(stat[i].name);
              
        var neo_link = document.createElement('button'); 
          
        var text = document.createTextNode(stat[i].name);
        var new_line = document.createElement("br");  
        neo_link.appendChild(text);
        neo_link.appendChild(new_line);
        neo_link.onclick = open_statistic(stat[i].id); 
        document.getElementById("statistics").appendChild(neo_link);
        console.log(i);
    }
    }
load();