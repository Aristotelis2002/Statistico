document.getElementById('login').addEventListener('submit',async function(e) {
    e.preventDefault(); //to prevent form submission
    let id = document.getElementById('userId').value
    const response = await fetch(
		`http://127.0.0.1:8080/stats/${id}`,
		{
			method: 'GET',
		}
	);
	const data = await response.json(); // Extracting data as a JSON Object from the response
    console.log(data);
    sessionStorage.setItem("stat",JSON.stringify(data));
    //storage["stat"] = data;
    window.location.href = "stats.html";
});