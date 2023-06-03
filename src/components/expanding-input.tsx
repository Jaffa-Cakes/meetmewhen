import { useState, useRef, useEffect, ChangeEvent } from 'react';

type Props = {
    placeholder: string;
    // In characters
    minWidth: number;
}

export default function ExpandingInput(props: Props) {
    const [inputValue, setInputValue] = useState('');
    const inputRef = useRef<HTMLInputElement>(null);
  
    const handleInputChange = (event: ChangeEvent<HTMLInputElement>) => {
      setInputValue(event.target.value);
    };
  
    useEffect(() => {
      if (inputRef.current) {
        inputRef.current.style.width = props.minWidth + 'ch';
        inputRef.current.style.width = `${inputRef.current.scrollWidth}px`;
      }
    }, [inputValue]);
  
    return (
      <div className="relative">
        <input
          ref={inputRef}
          className="block bg-transparent focus:outline-none focus:ring-0 font-medium"
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          placeholder={props.placeholder}
          autoFocus
        />
      </div>
    );
  };
