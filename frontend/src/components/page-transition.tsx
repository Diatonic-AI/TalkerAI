import React, { useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'

interface PageTransitionProps {
  children: React.ReactNode
}

export function PageTransition({ children }: PageTransitionProps) {
  const location = useLocation()
  const [isTransitioning, setIsTransitioning] = useState(false)
  const [displayLocation, setDisplayLocation] = useState(location)

  useEffect(() => {
    if (location !== displayLocation) {
      setIsTransitioning(true)
      
      // Short delay to show transition
      const timer = setTimeout(() => {
        setDisplayLocation(location)
        setIsTransitioning(false)
      }, 150)

      return () => clearTimeout(timer)
    }
  }, [location, displayLocation])

  return (
    <div
      className={`transition-all duration-200 ease-in-out ${
        isTransitioning 
          ? 'opacity-50 scale-[0.98]' 
          : 'opacity-100 scale-100'
      }`}
    >
      {children}
    </div>
  )
} 