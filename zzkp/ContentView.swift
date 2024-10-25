//
//  ContentView.swift
//  zzkp
//
//  Created by Oleg Lomaka on 23.10.2024.
//

import SwiftUI

struct ContentView: View {
    @State private var timestamp: String = ""
    @State private var inp: String = ""

    var body: some View {
        VStack(spacing: 20) {
            TextField("X", text: $inp)
            Text(timestamp)
                .padding()
                .border(Color.gray, width: 1)
                .frame(width: 300, height: 40, alignment: .center)
            Button {
                let currentTimestamp = Date().timeIntervalSince1970
                // timestamp = String(currentTimestamp)
                // print(timestamp)
                timestamp = "ts: \(currentTimestamp)"
                
                var cStringPointer: UnsafeMutablePointer<CChar>? = nil
                let ok = prove_pow(2, 3, &cStringPointer)
            
                print("Proving status: \(ok)")
                
            
                if let cString = cStringPointer {
                    print("addr in swift: \(cString)")
                    let receipt = String(cString: cString)
                    free(cStringPointer)

                    print("Got the result of length \(receipt.count)")
                    print("Result: \(receipt.prefix(10))")
                    
                    receipt.withCString { ptr in
                        let ok = receipt_verify(ptr)
                        print("Verification status: \(ok)")
                    }
                    
                } else {
                    print("Not a string")
                }
                
                
//                let receipt = "{}"
//                receipt.withCString { ptr in
//                    let ok = receipt_verify(ptr)
//                    print("OK: \(ok)")
//                }
            } label: {
                Text("Get Timestamp")
                    .padding()
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }

        }
        .padding()
    }
}

#Preview {
    ContentView()
}
