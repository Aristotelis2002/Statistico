var objects = sessionStorage.getItem("objects");
objects = JSON.parse(objects);
////TODO
//button incr dobavi object id kum nego
//button decr vsichko
//button delete
//button rename za imeto
//handler za ruchno promenqne
//button dobavi nov object
//button visualize graph/statistic
for (i in objects){
    const node = document.createTextNode(objects[i].name);
    var separate = document.createElement("div");
    var field = document.createElement('p');
    var counter = document.createElement("input");
    var button_incr = document.createElement("button");
    button_incr.innerHTML="+";
    button_incr.className = "button-incr";
    counter.type = "text";
    counter.value = objects[i].counter;
    counter.className = "counter-class";
    field.appendChild(node);
    separate.appendChild(field);
    separate.appendChild(counter);
    separate.appendChild(button_incr);
    document.getElementById("objects").appendChild(separate);
   // document.getElementById("objects").appendChild(counter);
}