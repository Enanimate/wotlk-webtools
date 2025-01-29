"use client"

export default function login() {
    const submitData = async (event) => {
        event.preventDefault()
    
        const formData = new FormData(event.currentTarget);
        const username = formData.get('username');
        const password = formData.get('password');
    
        let response = await fetch('/api/auth', {
          method: 'POST',
          body: JSON.stringify({
            username,
            password,
          }),
          headers: {
            'Content-type': 'application/json'
          }
        })
      }


    return (
        <div className="flex justify-center p-16 w-full min-h-screen">
            <div className="flex flex-col gap-10 w-[32rem] h-[42rem]">
                <h1 className="font-bold text-xl">Login</h1>
                <form className="flex flex-col gap-4" onSubmit={submitData} method="post">
                    <input
                        type="username"
                        name="username"
                        placeholder="Username"
                        required
                    />
                    <input
                        type="password"
                        name="password"
                        placeholder="Password"
                        required
                    />
                    <button type="submit">Login</button>
                </form>
            </div>
        </div>
    );
}