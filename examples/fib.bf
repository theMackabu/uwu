// Brainfuck Fibonacci calculation
// Forked from pablomm/fibonacci_bf
+>                                                          // Set #1 to 1; Shift to #2
+>                                                          // Set #2 to 1l Shift to #3 (CLP)

<<                                                          // Shift to #1 for output loop of #1 and #2
[
++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++ . ---------- ---------- ---------- ---------- -------->                      // Add 48; Output ASCII; Decrease 48: shift right 1
>>>>>>>>>> ++++++++++ ++++++++++ ++++++++++ ++ . ---------- ---------- ---------- -- <<<<<<<<<<                                 // Shift to #10; Outpur ASCII 32 as SPACE
]

++++++++++ ++++++++++ ++++++++++ ++++++++++ +++++          // Set #3 Counter Loop Register to 45
                                                            // 45 works for 32 bit memory only
                                                            // Set to 11 for 8 bit
[
    -                       // Decrement current Counter Loop Register (CLP) at #3 by 1
        [->+<]              // Move CLP forward to Position Register (POR) at #4 ; End Loop on Empty CLP; POR becomes next CLP
    <<                      // Shift back to First Adend (FA)
        [->>>>+>+<<<<<]     // Move FA to First Adend Temp (FAT) at #5 and First Adend Memory (FAM) at #6; End on Empty FA
    >                       // Shift to Second Adend (SA)
        [->>>>>+>+<<<<<<]   // Move SA to Second Adend Temp (SAT) at #7 and Second Adend Memory (SAM) at #8; End on Empty SA
    >>>                     // Shift to FAT
        [-<<<<+>>>>]        // Move FAT back to Original FA postion; End on Emtpy FAT
    >                       // Shift to SAT
        [-<<<+>>>]          // Move SAT to Original SA postion; End on Empyt SAT
    >                       // Shift to FAM
        [-<<<<<+>>>>>]      // Add FAM to New Addend Position (NAP) at old CLP location
    >                       // Shift to SAM
        [-<<<<<+>>>>>]      // Add SAM to NAP; End on empty SAM
    <<<<<                    // Shift to new NAP;

// Code taken from esolangs_org/wiki/brainfuck_algorithms
// copy cell to workspace and back

[>>>>>>>+>+<<<<<<<<-]>>>>>>>>
[<<<<<<<<+>>>>>>>>-]<<+>
[<->[                                                       // while value exists
   >++++++++++<                                             // make a 10
   [->-[>+>>]>[+[-<+>]>+>>]<<<<<]                           // divide value by 10
   >[-]                                                     // dont need this cell
   ++++++++[<++++++>-]                                      // add 48 to remainder
   >[<<+>>-]                                                // store the remainder
   >[<<+>>-]                                                // get next value
   <<
]>]
<[-                                                         // else need to make a zero
   >>++++++++[<++++++>-]
]
<[.[-]<]<<<<<                                               // print and clear each stored remainder in reverse

>>>>>>>>>> ++++++++++ ++++++++++ ++++++++++ ++ . ---------- ---------- ---------- -- <<<<<<<<<<                         // Shift forward 10 registers; Output SPACE as ASCII 32; Shift back 10
]