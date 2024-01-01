```
┌─────────────────┐              
│    Tokenizer    │        ───   
└─────────────────┘         │    
         │                  │    
┌─────────────────┐         │    
│     Parser      │      Frontend
└─────────────────┘         │    
         │                  │    
┌─────────────────┐         │    
│ Code Generator  │        ───   
└─────────────────┘              
         │                       
┌─────────────────┐              
│ Virtual Machine │        ───   
└─────────────────┘         │    
         │                  │    
┌─────────────────┐         │    
│     B-Tree      │         │    
└─────────────────┘         │    
         │               Backend 
┌─────────────────┐         │    
│      Pager      │         │    
└─────────────────┘         │    
         │                  │    
┌─────────────────┐         │    
│  OS Interface   │        ───   
└─────────────────┘              
```