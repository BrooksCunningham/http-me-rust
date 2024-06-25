document.addEventListener("securitypolicyviolation", function(e) {
    
    alert("security policy violation!");
})

addEventListener("securitypolicyviolation", (event) => {
    console.log(event);
});