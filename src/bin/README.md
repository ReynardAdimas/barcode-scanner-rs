# Accuracy Summary 
## Setup 
The experiment tested 6 decoders configurations againts 5 global preprocessing variants, using a dataset of 15 symbology folders from `test-images/`, each expanced into 6 synthetic conditions (clean, canvas4x, blur, rotation, low-contrast, noise) and giving 90 test cases per preprocessing variant. 

## Decoders tested:
- `opencv-qr`: OpenCv native QR-only detector 
- `opencv-id` : OpenCv native 1D barcode detector 
- `wechat` : WeChat QR detector (CNN-based, includes super-resolution)
- `rxing` : pure Rust decoder, supports both QR and 1D 
- `hybrid` : fallback chain: opencv-qr --> wechat --> rxing 
- `hybrid+1d` : fallback chain: opencv=qr --> opencv-id --> wechat ->> rxing 

## Preprocessing variants tested:
- `raw` : no processing 
- 'gray` : grayscale only 
- 'gray_clahe` : grayscale + local contrast enchancement (CLAHE)
- `gray_clahe_blur` : grayscale + CLAHE + light Gaussian blur 
- `gray_otsu` : grayscale + global Otsu binarization 

All 6 decoders received the identical preprocessed image per case, ensuring any accuracy/latency difference reflects decoder capability, not input variation. 

## Finding 1: Single-format decoders score low by design
`opencv-qr` (~5.6% ok) and `wechat` (~6.7% ok) look weak accross the full 90-case summary, but this is expected: only ~1 or 15 folders is QR-symbology, so ~94% of test cases fall outside their supported scope. Their real performance should be judged from the coverage matrix (per-folder, decoder-vs-symbology), not this aggregate table. 


