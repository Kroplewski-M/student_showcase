import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import { GoogleAnalytics } from "@next/third-parties/google";
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
  title: "SCE Futures '26",
  description: "SCE Futures '26 Student Showcase @ University Of Huddersfield",
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
      <head>
        <link rel="icon" href="/favicon.svg" sizes="any" />
        <script
          async
          src="https://www.googletagmanager.com/gtag/js?id=G-SQ11DTF14N"
        ></script>
      </head>
      <body className={`${poppins.variable} antialiased overflow-x-hidden`}>
        <AuthProvider initialUser={user}>
          <Nav />
          {children}
          <Footer />
        </AuthProvider>
      </body>
      <GoogleAnalytics gaId="G-SQ11DTF14N" />
    </html>
  );
}
