const getTodo = () => {
    setInterval(async () => {
        try {
            const response = await fetch('/todos/all').then((r) => r.json());
            let todos = "";
            response.forEach(todo => {
                todos += `<li>
                    id: ${todo.id} </br>
                    title: ${todo.title} </br>
                    description: ${todo.description} </br>
                    status: ${todo.status} </br>
               </li> </br>`;
            });
            document.querySelector("#todo-list").innerHTML = todos;
        } catch (err) {
            console.log(err);
        }
    }, 1000);
};

getTodo();

document.querySelector('#new-todo').addEventListener('submit', async (e) => {
    const formData = new FormData(e.target);
    const data = {
        title: formData.get('title'),
        description: formData.get('description')
    };
    try {
        await fetch('/todo/create', {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });
    } catch (err) {
        console.log(err);
    }
});

document.querySelector('#update-status').addEventListener('submit', async (e) => {
    const formData = new FormData(e.target);
    const id = Number(formData.get('id'));
    try {
        await fetch(`/todo/${id}/mark/completed`, { method: "PUT" });
    } catch (err) {
        console.log(err);
    }
});

document.querySelector('#delete-todo').addEventListener('submit', async (e) => {
    const formData = new FormData(e.target);
    const id = Number(formData.get('id'));
    try {
        await fetch(`/todo/${id}/delete`, { method: "DELETE" });
    } catch (err) {
        console.log(err);
    }
});