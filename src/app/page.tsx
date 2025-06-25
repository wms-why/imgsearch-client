'use client'
import { useEffect, useState } from "react";
import GuidePage from "./guide/page";
import SearchPage from "./search/page"
import { get, GuideType } from "@/data/guide";

export default function App() {

  const [guide, setGuide] = useState<GuideType>();

  useEffect(() => {
    get().then(guide => setGuide(guide));
  })

  return guide === 'finished' ? <SearchPage /> : <GuidePage />;
}