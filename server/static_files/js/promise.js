async function getResponse() {
    let response = await fetch ("url")
    let content = await response.json()
    content = content.splice(0,10)

    let list = document.querySelector(".posts")

    let key;

    //for in
    for (key in content) {

        list.innerHTML += 
        //"content"
        
    }

}

getResponse()