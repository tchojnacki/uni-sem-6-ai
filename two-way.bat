cargo build --release -p game-theory

cd target/release
copy nul 1to2.txt >nul
copy nul 2to1.txt >nul
cls
step.exe -p p2 <1to2.txt >>2to1.txt | step.exe -p p1 <2to1.txt >>1to2.txt
del 1to2.txt 2to1.txt
cd ../..
