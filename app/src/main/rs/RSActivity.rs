#pragma  version (1)
#pragma  rs  java_package_name(com.example.imageview1)

//Variables globales

//Utilisée dans colorize()
float hue = 0;

//Utilisée dans toGray()
static  const  float4  weight = {0.299f, 0.587f, 0.114f, 0.0f};

//Utilisée dans toGrayExceptOneColor()
float color = 0;

//Utilisées dans fillHisto(), dimConrs()
volatile int32_t histogram[255];
float minI=255;
float maxI=0;
//Utilisées dans egalHisto();
int width=0;
int height=0;
//Retourne un tableau des valeurs hsv correspondant au pixel in
float3 RS_KERNEL getHSV(uchar4 in ) {

 const float4 f4 = rsUnpackColor8888( in );

 float r = f4.r;
 float g = f4.g;
 float b = f4.b;

 float minRGB = min(r, min(g, b));
 float maxRGB = max(r, max(g, b));
 float deltaRGB = maxRGB - minRGB;

 float h = 0.0;
 float s = maxRGB == 0 ? 0 : (maxRGB - minRGB) / maxRGB;
 float v = maxRGB;

 if (deltaRGB != 0) {

  if (r == maxRGB) {
   h = (g - b) / deltaRGB;
  } else {
   if (g == maxRGB) {
    h = 2 + (b - r) / deltaRGB;
   } else {
    h = 4 + (r - g) / deltaRGB;
   }
  }

  h *= 60;
  if (h < 0) {
   h += 360;
  }
  if (h == 360) {
   h = 0;
  }
 }
 float3 hsv = {
  h,
  s,
  v
 };
 return hsv;

}

//Retourne un pixel aux valeurs RGB correspondant au tableau HSV
uchar4 RS_KERNEL HSVToUCHAR4(float3 hsv) {
 float r = 0;
 float g = 0;
 float b = 0;

 float h = hsv.x;
 float s = hsv.y;
 float v = hsv.z;

 int ti = (int) fmod((h / 60), 6);
 float f = (h / 60) - ti;
 float l = v * (1 - s);
 float m = v * (1 - f * s);
 float n = v * (1 - f) * s;
 if (ti == 0) {
  r = v;
  g = n;
  b = l;
 }
 if (ti == 1) {
  r = m;
  g = v;
  b = 1;
 }
 if (ti == 2) {
  r = 1;
  g = v;
  b = n;
 }
 if (ti == 3) {
  r = 1;
  g = m;
  b = v;
 }
 if (ti == 4) {
  r = n;
  g = 1;
  b = v;
 }
 if (ti == 5) {
  r = v;
  g = 1;
  b = m;
 }

 return rsPackColorTo8888(r, g, b);
}



//Retourne le pixel in grisé
uchar4 RS_KERNEL toGray(uchar4 in ) {
 const float4 pixelf = rsUnpackColor8888( in );
 const float gray = dot(pixelf, weight);
 return rsPackColorTo8888(gray, gray, gray, pixelf.a);
}

//Retourne le pixel in après modification de sa teinte en la variable globale hue
uchar4 RS_KERNEL colorize(uchar4 in ) {
 //Convert input uchar4 to float4
 float4 f4 = rsUnpackColor8888( in );
 // convert rgb to hsv
 float3 hsv = getHSV( in );
 hsv.x = hue;
 uchar4 rgb = HSVToUCHAR4(hsv);
 return rgb;
}


//Retourne un pixel grisé si sa teinte n'est pas proche de la variabl globale color
uchar4 RS_KERNEL toGrayExceptOneColor(uchar4 in ) {
 const float4 f4 = rsUnpackColor8888( in );
 const float gray = dot(f4, weight);

 // convert rgb to hsv
 float r = f4.r;
 float g = f4.g;
 float b = f4.b;

 float3 hsv = getHSV( in );
 float h = hsv.x;
 if (h < color - 30 || h > color + 30) {
  return rsPackColorTo8888(gray, gray, gray, f4.a);
 }

 return rsPackColorTo8888(r, g, b);


}




//Modifie la valeur des variables globales minI ou maxI en fonction du niveau de gris du pixel in
void RS_KERNEL minMax(uchar4 in ) {
 const float4 pixelf = rsUnpackColor8888( in );
 const float gray = dot(pixelf, weight);

 int g = gray * 255;

 if (g < minI) {
  minI = g;
 }
 if (g > maxI) {
  maxI = g;
 }
}

//Incrémente la case de l'hitogramme correspondant au niveau de gris du pixel in
void RS_KERNEL fillHisto(uchar4 in ) {
 uchar4 gray = toGray( in );
 volatile int32_t * addr = & histogram[gray.r];
 //rsDebug("",histogram[gray.r]);
 rsAtomicInc(addr);
}



//Modifie le niveau de gris du pixel in de manière à égaliser l'histogramme des niveaux de gris
uchar4 RS_KERNEL egalHisto(uchar4 in ) {

 int value = 100;
 double contrast = ((100 + value) / 100, 2) * ((100 + value) / 100, 2);
 uchar4 gray = toGray( in );
 int R = gray.r;

 R = (int)(((((R / 255.0) - 0.5) * contrast) + 0.5) * 255.0);
 if (R < 0) {
  R = 0;
 } else if (R > 255) {
  R = 255;
 }

 int newI = (histogram[R] * 255) / (width * height);
 return rsPackColorTo8888(newI, newI, newI);
}

//Modifie le niveau de gris du pixel in de manière à augmenter le contraste de l'image
uchar4 RS_KERNEL ELDDrs(uchar4 in ) {
 int value = 100;
 double contrast = ((100 + value) / 100, 2) * ((100 + value) / 100, 2);
 // convert rgb to hsv
 float R = in .r;

 R = (int)(((((R / 255.0) - 0.5) * contrast) + 0.5) * 255.0);
 if (R < 0) {
  R = 0;
 } else if (R > 255) {
  R = 255;
 }

 uchar4 result = rsPackColorTo8888(R, R, R);
 return result;
}

//Modifie le niveau de gris du pixel in de manière à diminuer le contraste de l'image
uchar4 RS_KERNEL dimConrs(uchar4 in ) {

 int difference = 10;
 minI = minI + difference;
 maxI = maxI - difference;

 float gr = in .r;
 float newI = gr;
 uchar4 result;
 if (newI > 0 + difference && newI < 255 - difference) {
  result = rsPackColorTo8888(newI / 255, newI / 255, newI / 255);
 } else {
  if (newI < 255 - difference) {
   result = rsPackColorTo8888(0 + difference, 0 + difference, 0 + difference);
  } else {
   if (newI > 0 + difference) {
    result = rsPackColorTo8888(255 - difference, 255 - difference, 255 - difference);
   }
  }
 }
 return result;
}

//Modifie les valeurs RGB du pixel in de manière à augmenter le contraste de l'image
uchar4 RS_KERNEL ELDDColor(uchar4 in ) {
 int value = 100;
 double contrast = ((100 + value) / 100, 2) * ((100 + value) / 100, 2);
 // convert rgb to hsv
 float R = in .r;
 float G = in .g;
 float B = in .b;

 R = (int)(((((R / 255.0) - 0.5) * contrast) + 0.5) * 255.0);
 if (R < 0) {
  R = 0;
 } else if (R > 255) {
  R = 255;
 }

 G = (int)(((((G / 255.0) - 0.5) * contrast) + 0.5) * 255.0);
 if (G < 0) {
  G = 0;
 } else if (G > 255) {
  G = 255;
 }

 B = (int)(((((B / 255.0) - 0.5) * contrast) + 0.5) * 255.0);
 if (B < 0) {
  B = 0;
 } else if (B > 255) {
  B = 255;
 }

 uchar4 result = rsPackColorTo8888(R, G, B);
 return result;
}