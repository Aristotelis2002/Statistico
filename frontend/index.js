//import { ADDRESS } from "./env.js"
document.getElementById('login').addEventListener('submit',async function(e) {
    e.preventDefault(); //to prevent form submission
    let id = document.getElementById('userId').value;// ; added
    const response = await fetch(
		`${ADDRESS}/stats/${id}`,
		{
			method: 'GET',
		}
	);
	const data = await response.json(); // Extracting data as a JSON Object from the response
    console.log(data);
    sessionStorage.setItem("stat",JSON.stringify(data));
    sessionStorage.setItem("userId",JSON.stringify(id));
    //storage["stat"] = data;
    window.location.href = "stats.html";
});