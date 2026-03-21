"use client";
import { useEffect } from "react";

interface StudentsResultProps {
  query: string;
}

export default function StudentsResult({ query }: StudentsResultProps) {
  useEffect(() => {
    console.log(query);
  }, [query]);
  return <div className=""></div>;
}
