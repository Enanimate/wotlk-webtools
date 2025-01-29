import Link from "next/link";

const endpoints = [
    { name: "Home", path: "/" },
    { name: "Download", path: "/download" },
    { name: "Admin", path: "/admin" }
]
export default function NavBar() {
    return(
        <nav className="flex justify-center items-center gap-10 bg-[#102133] h-16 w-screen shadow-2xl">
            {endpoints.map((point) => (
                <Link key={point.name} href={point.path} className="h-16 min-w-10 px-6 pt-6 text-[#C7AA1A] font-bold hover:shadow-[inset_0px_-2px_#D2811E]">{point.name}</Link>
            ))}
        </nav>
    );
}