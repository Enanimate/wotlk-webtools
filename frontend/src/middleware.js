import { NextResponse } from "next/server"

export function middleware(request) {
  const user = 'd';

  if (!user) {
    return NextResponse.redirect(
      new URL('/login', request.url)
    )
  };
}

export const config = {
  matcher: ['/admin', '/download']
};