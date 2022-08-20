var stat = sessionStorage.getItem("stat");
stat = JSON.parse(stat);
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
//podai imeto na statistikata kum href
//Button suzdai nova statistika
//button iztrii stara statistika
//button rename statistika
for(var i in stat){
    const node = document.createTextNode(stat[i].name);
          
    var neo_link = document.createElement('button'); 
      
    var text = document.createTextNode(stat[i].name);
      
    neo_link.appendChild(text);
    neo_link.onclick = open_statistic(stat[i].id); 
    document.getElementById("statistics").appendChild(neo_link);
}