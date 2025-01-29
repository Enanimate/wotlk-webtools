import { NextResponse } from "next/server"

export async function POST(request) {
    const body = await request.json();

    let response = await fetch('http://127.0.0.1:3001/', {
        method: 'POST',
        body: JSON.stringify({
            username: body.username,
            password: body.password,
        }),
        headers: {
            'Content-type': 'application/json'
        }
    })
    .then(response => response.json())
    .then(result => {
        console.log(result)
    })

    return NextResponse.json({ status: 200, message: "Received", data: response })
}

/*
const submitData = async (event) => {
        event.preventDefault()
    
        const formData = new FormData(event.currentTarget);
        const username = formData.get('username');
        const password = formData.get('password');
    
        let response = await fetch('http://127.0.0.1:3001/', {
          method: 'POST',
          body: JSON.stringify({
            username,
            password,
          }),
          headers: {
            'Content-type': 'application/json'
          }
        })
        .then(response => response.json())
        .then(result => {
            console.log(result)
        })
      }
*/