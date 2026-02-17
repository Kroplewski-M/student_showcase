import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import "./globals.css";
import "@fortawesome/fontawesome-svg-core/styles.css";
import { config } from "@fortawesome/fontawesome-svg-core";
import Footer from "./components/Footer";
import { getUser } from "./lib/auth";
import { AuthProvider } from "./context/auth-context";
import Nav from "./components/Nav";

config.autoAddCss = false;
const poppins = Poppins({
  subsets: ["latin"],
  weight: ["100", "200", "300", "400", "500", "600", "700", "800", "900"],
  variable: "--font-poppins",
});

export const metadata: Metadata = {
  title: "C&E Futures '26",
  description: "C&E Futures '26 Student Showcase @ University Of Huddersfield",
  other: {
    "mobile-web-app-capable": "yes",
    "apple-mobile-web-app-capable": "yes",
    "apple-mobile-web-app-status-bar-style": "black-translucent",
  },
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const user = await getUser();
  return (
    <html lang="en">
      <body className={`${poppins.variable} antialiased`}>
        <AuthProvider initialUser={user}>
          <Nav />
          {children}
          <Footer />
        </AuthProvider>
      </body>
    </html>
  );
}
