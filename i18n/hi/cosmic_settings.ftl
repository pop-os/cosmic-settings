app = कास्मिक सेटिंग्स
dbus-connection-error = DBus से कनेक्ट करने में विफल
ok = ठीक है
unknown = अज्ञात
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] तार
        [wifi] वाई-फाई
        [vpn] वी.पी.एन
       *[other] अज्ञात
    } कनेक्शन और प्रोफाइल।
add-network = नेटवर्क जोड़ें
    .profile = प्रोफाइल जोड़ें
add-vpn = वी.पी.एन जोड़ें
airplane-on = एयरप्लेन मोड चालू है।
cable-unplugged = तार अनप्लग किया गया है
connect = कनेक्ट करें
connected = कनेक्टेड
connecting = कनेक्ट किया जा रहा है…
disconnect = कनेक्शन काटें
forget = भूल जाओ
known-networks = ज्ञात नेटवर्क
network-and-wireless = नेटवर्क और वायरलेस
no-networks = कोई नेटवर्क नहीं मिला।
no-vpn = कोई वी.पी.एन कनेक्शन नहीं है।
password = पासवर्ड
remove = हटा दें
settings = सेटिंग्स
username = उपयोगकर्ता नाम
visible-networks = दृश्य नेटवर्क
auth-dialog = प्रमाणीकरण आवश्यक है
    .vpn-description = वी.पी.एन सेवा के लिए आवश्यक उपयोगकर्ता नाम और पासवर्ड दर्ज करें।
    .wifi-description = पासवर्ड या सुरक्षा कुंजी दर्ज करें। आप राउटर पर "WPS" बटन दबाकर भी कनेक्ट कर सकते हैं।
forget-dialog = क्या आप इस वाई-फाई नेटवर्क को भूलना चाहते हैं?
    .description = आपको भविष्य में इस वाई-फाई नेटवर्क का उपयोग करने के लिए फिर से पासवर्ड दर्ज करना होगा।
network-device-state =
    .activated = कनेक्टेड
    .config = कनेक्ट किया जा रहा है
    .deactivating = कनेक्शन काटा जा रहा है
    .disconnected = कनेक्शन कट गया है
    .failed = कनेक्ट करने में विफल
    .ip-check = कनेक्शन की जांच की जा रही है
    .ip-config = आईपी और राउटिंग जानकारी मांगी जा रही है
    .need-auth = प्रमाणीकरण आवश्यक है
    .prepare = कनेक्ट करने के लिए तैयार हो रहा है
    .secondaries = द्वितीयक कनेक्शन के लिए इंतजार कर रहा है
    .unavailable = अनुपलब्ध
    .unknown = अज्ञात स्थिति
    .unmanaged = प्रबंधित नहीं
    .unplugged = तार अनप्लग किया गया है
remove-connection-dialog = क्या आप कनेक्शन प्रोफाइल हटाना चाहते हैं?
    .vpn-description = आपको भविष्य में इस नेटवर्क का उपयोग करने के लिए फिर से पासवर्ड दर्ज करना होगा।
    .wired-description = आपको भविष्य में इसे उपयोग करने के लिए प्रोफाइल को फिर से बनाना होगा।
vpn = VPN
    .connections = VPN कनेक्शन
    .error = VPN कॉन्फ़िगरेशन जोड़ने में विफल
    .remove = कनेक्शन प्रोफ़ाइल हटाएं
    .select-file = एक VPN कॉन्फ़िगरेशन फ़ाइल चुनें
vpn-error = VPN त्रुटि
    .config = VPN कॉन्फ़िगरेशन जोड़ने में विफल
    .connect = VPN से कनेक्ट करने में विफल
    .connection-editor = कनेक्शन संपादक विफल
    .connection-settings = सक्रिय कनेक्शनों के लिए सेटिंग्स प्राप्त करने में विफल
    .updating-state = नेटवर्क प्रबंधक की स्थिति अपडेट करने में विफल
    .wireguard-config-path = वायरगार्ड कॉन्फ़िग फ़ाइल पथ अमान्य है
    .wireguard-config-path-desc = चयनित फ़ाइल स्थानीय फाइल सिस्टम पर होनी चाहिए।
    .wireguard-device = वायरगार्ड डिवाइस बनाने में विफल
    .with-password =
        nmcli के साथ VPN { $field ->
           *[username] उपयोगकर्ता नाम
            [password] पासवर्ड
            [password-flags] पासवर्ड-फ्लैग्स
        } सेट करने में विफल
wired = तार
    .adapter = तार एडाप्टर { $id }
    .connections = तार कनेक्शन
    .devices = तार उपकरण
    .remove = कनेक्शन प्रोफाइल हटाएं
wifi = वाई-फाई
    .adapter = वाई-फाई एडाप्टर { $id }
    .forget = इस नेटवर्क को भूल जाओ
wireguard-dialog = वायरगार्ड डिवाइस जोड़ें
    .description = वायरगार्ड कॉन्फ़िग के लिए एक डिवाइस नाम चुनें।

## Networking: Online Accounts

online-accounts = ऑनलाइन खातें
    .desc = खातें जोड़ें, IMAP और SMTP, नौकरी लॉगिन

# Bluetooth

confirm = पुष्टि करें
bluetooth = ब्लूटूथ
    .desc = ब्लूटूथ उपकरण प्रबंधित करें
    .status = जब ब्लूटूथ सेटिंग्स खुलती हैं, तो यह सिस्टम { $aliases } के रूप में दिखाई देता है।
    .connected = कनेक्टेड
    .connecting = कनेक्ट किया जा रहा है
    .disconnecting = कनेक्शन काटा जा रहा है
    .connect = कनेक्ट करें
    .disconnect = कनेक्शन काटें
    .forget = भूल जाओ
    .dbus-error = DBus में त्रुटि हुई: { $why }
    .disabled = ब्लूटूथ सेवा अक्षम है
    .inactive = ब्लूटूथ सेवा सक्रिय नहीं है
    .unknown = ब्लूटूथ सेवा सक्रिय नहीं की जा सकी। क्या BlueZ इंस्टॉल किया गया है?
bluetooth-paired = पहले कनेक्ट किए गए उपकरण
    .connect = कनेक्ट करें
    .battery = { $percentage }% बैटरी
bluetooth-confirm-pin = ब्लूटूथ पिन की पुष्टि करें
    .description = कृपया पुष्टि करें कि निम्नलिखित पिन { $device } पर प्रदर्शित पिन से मेल खाता है
bluetooth-available = पास के उपकरण
bluetooth-adapters = ब्लूटूथ एडाप्टर

## Desktop

desktop = डेस्कटॉप

## Desktop: Wallpaper

wallpaper = वॉलपेपर
    .change = चित्र को बदलें
    .desc = वॉलपेपर चित्र, रंग और स्लाइड शो विकल्प।
    .fit = वॉलपेपर समायोजन
    .folder-dialog = वॉलपेपर फ़ोल्डर चुनें
    .image-dialog = वॉलपेपर चित्र चुनें
    .plural = वॉलपेपर
    .same = सभी डिस्प्ले पर एक ही वॉलपेपर
    .slide = स्लाइड शो
add-color = रंग जोड़ें
add-image = चित्र जोड़ें
all-displays = सभी डिस्प्ले
colors = रंग
dialog-add = जोड़ें
fill = भरें
fit-to-screen = स्क्रीन पर समायोजन
open-new-folder = नया फ़ोल्डर खोलें
recent-folders = हाल के फ़ोल्डर
x-minutes = { $number } मिनट
x-hours =
    { $number ->
        [1] 1 घंटा
       *[other] { $number } घंटे
    }

## Desktop: Appearance

appearance = रूपरेखा
    .desc = प्राधिकरण रंग और थीम।
accent-color = प्राधिकरण रंग
app-background = एप्लिकेशन या विंडो का पृष्ठभूमि
auto = स्वचालित
close = बंद करें
color-picker = रंग चयनकर्ता
copied-to-clipboard = क्लिपबोर्ड में कॉपी किया गया
copy-to-clipboard = क्लिपबोर्ड में कॉपी करें
dark = गहरा
export = निर्यात
hex = हेक्स
import = आयात
light = हल्का
mode-and-colors = मोड और रंग
recent-colors = हाल के रंग
reset-to-default = डिफ़ॉल्ट पर पुनर्स्थापित करें
rgb = RGB
window-hint-accent = सक्रिय विंडो संकेतक रंग
window-hint-accent-toggle = सक्रिय विंडो संकेतक के अनुसार थीम उच्चारण रंग का उपयोग करें
auto-switch = हल्के और गहरे मोड में स्वचालित रूप से बदलें
    .sunrise = सूर्योदय पर हल्के मोड में बदल जाता है
    .sunset = सूर्यास्त के समय गहरे मोड में बदल जाता है
    .next-sunrise = अगले सूर्योदय पर हल्के मोड में बदल जाता है
    .next-sunset = अगले सूर्यास्त के समय गहरे मोड में बदल जाता है
container-background = कंटेनर पृष्ठभूमि
    .desc-detail = नेविगेशन साइडबार, साइड ड्रावर, संवाद और अन्य विजेट के लिए कंटेनर पृष्ठभूमि रंग का उपयोग किया जाता है। पूर्वनिर्धारित रूप से, यह स्वचालित रूप से एप्लिकेशन या विंडो की पृष्ठभूमि से प्राप्त किया जाता है।
    .reset = पुनर्स्थापित करें
    .desc = नेविगेशन साइडबार, साइड ड्रावर, संवाद और अन्य साधारण विस्तार के लिए उपयोग की जाने वाली प्रमुख कंटेनर रंग।
control-tint = नियंत्रण घटक रंग
    .desc = मानक बटन, खोज इनपुट, पाठ इनपुट और अन्य ऐसे घटकों की पृष्ठभूमियों के लिए उपयोग किया जाता है।
frosted = प्रबंधकीय इंटरफ़ेस में फ्रॉस्टेड ग्लास प्रभाव
    .desc = पैनल, डॉक, एप्लेट, लॉन्चर और एप्लिकेशन पुस्तकालय के पृष्ठभूमि पर धुंधला प्रभाव लागू होता है।
enable-export = इस थीम को GNOME एप्लिकेशन पर लागू करें।
    .desc = सभी उपकरणों द्वारा स्वचालित परिवर्तनों का समर्थन किया गया है यह कहना संभव नहीं है। नॉन-COSMIC एप्लिकेशन को थीम परिवर्तन के बाद फिर से शुरू करना आवश्यक है।
icon-theme = आइकन थीम
    .desc = एप्लिकेशन के लिए विभिन्न आइकन लागू करता है।
text-tint = इंटरफेस पाठ का टिंट
    .desc = विभिन्न स्तर पर पर्याप्त विविधता के बिना इंटरफेस पाठ रंग उत्पन्न करने के लिए उपयोग किया जाने वाला रंग।
style = शैली
    .round = गोल
    .slightly-round = थोड़ी गोल
    .square = चौकोर
interface-density = इंटरफेस घनत्व
    .comfortable = आरामदायक
    .compact = संकुचित
    .spacious = विशाल
window-management-appearance = विंडो प्रबंधन
    .active-hint = सक्रिय विंडो संकेतक का आकार
    .gaps = टाइल की गई विंडो के चारों ओर खाली स्थान

### Experimental

experimental-settings = प्रायोगिक सेटिंग्स
icons-and-toolkit = आइकन और टूलकिट थीमिंग
interface-font = सिस्टम फ़ॉन्ट
monospace-font = मोनोस्पेस फ़ॉन्ट

## Desktop: Notifications

notifications = सूचनाएँ
    .desc = डी.एन.डी, लॉक स्क्रीन सूचनाएँ, और प्रत्येक एप्लिकेशन सेटिंग्स।

## Desktop: Panel

panel = पैनल
    .desc = डेस्कटॉप नियंत्रण और मेन्यू के साथ ऊपरी सूची।
add = जोड़ें
add-applet = ऐपलेट जोड़ें
all = सभी
applets = ऐपलेट्स
center-segment = केंद्र खंड
drop-here = ऐपलेट्स को यहाँ छोड़ें
end-segment = अंतिम खंड
large = बड़ा
no-applets-found = कोई ऐपलेट्स नहीं मिले...
panel-bottom = नीचे
panel-left = बाएं
panel-right = दाएं
panel-top = शीर्ष
search-applets = ऐपलेट्स खोजें...
small = छोटा
start-segment = प्रारंभ खंड
panel-appearance = रूप
    .match = डेस्कटॉप से मेल खाएँ
    .light = हल्का
    .dark = गहरा
panel-behavior-and-position = व्यवहार और स्थिति
    .autohide = पैनल को स्वचालित रूप से छिपाएँ
    .dock-autohide = डॉक को स्वचालित रूप से छिपाएँ
    .position = स्क्रीन पर स्थिति
    .display = डिस्प्ले में दिखाएँ
panel-style = शैली
    .anchor-gap = पैनल और स्क्रीन के किनारों के बीच का अंतर
    .dock-anchor-gap = डॉक और स्क्रीन के किनारों के बीच का अंतर
    .extend = स्क्रीन के किनारों तक पैनल बढ़ाएँ
    .dock-extend = स्क्रीन के किनारों तक डॉक बढ़ाएँ
    .appearance = रूप
    .size = आकार
    .background-opacity = पृष्ठभूमि की पारदर्शिता
panel-applets = कॉन्फ़िगरेशन
    .dock-desc = डॉक ऐपलेट्स कॉन्फ़िगर करें
    .desc = पैनल ऐपलेट्स कॉन्फ़िगर करें
panel-missing = पैनल कॉन्फ़िगरेशन अनुपस्थित है
    .desc = कस्टम कॉन्फ़िगरेशन के कारण पैनल कॉन्फ़िगरेशन फ़ाइल अनुपस्थित है या यह दोषपूर्ण है।
    .fix = डिफ़ॉल्ट पर पुनर्स्थापित करें

## Desktop: Dock

dock = डॉक
    .desc = ऐप्लिकेशन ट्रे और अन्य ऐपलेट्स में पिन किए गए ऐप्स के साथ पैनल।

## Desktop: Window management

window-management = विंडो प्रबंधन
    .desc = सुपर कुंजी क्रियाएँ, विंडो नियंत्रण विकल्प और अतिरिक्त विंडो टाइलिंग विकल्प।
super-key = सुपर कुंजी क्रिया
    .launcher = लांचर खोलें
    .workspaces = कार्यक्षेत्र खोलें
    .applications = ऐप्लिकेशन खोलें
    .disable = अस्वीकृत करें
window-controls = विंडो नियंत्रण
    .maximize = अधिकतम करें बटन दिखाएँ
    .minimize = न्यूनतम करें बटन दिखाएँ
    .active-window-hint = सक्रिय विंडो संकेत दिखाएं
focus-navigation = फ़ोकस नेविगेशन
    .focus-follows-cursor = फ़ोकस कर्सर का अनुसरण करता है
    .focus-follows-cursor-delay = फ़ोकस कर्सर का अनुसरण करने में देरी (मिलीसेकंड में)
    .cursor-follows-focus = कर्सर फ़ोकस का अनुसरण करता है

## Desktop: Workspaces

workspaces = कार्यक्षेत्र
    .desc = कार्यक्षेत्र का पूर्वावलोकन और व्यवहार।
workspaces-behavior = कार्यक्षेत्र का व्यवहार
    .dynamic = डायनामिक कार्यक्षेत्र
    .dynamic-desc = खाली कार्यक्षेत्र स्वचालित रूप से हटा दिए जाते हैं।
    .fixed = कार्यक्षेत्रों की निश्चित संख्या
    .fixed-desc = सर्वेक्षण में कार्यक्षेत्र जोड़ें या हटा दें।
workspaces-multi-behavior = मल्टी-मॉनिटर व्यवहार
    .span = कार्यक्षेत्र डिस्प्ले में फैले होते हैं
    .separate = डिस्प्ले के लिए अलग कार्यक्षेत्र होते हैं
workspaces-overview-thumbnails = कार्यक्षेत्र के पूर्वावलोकन थंबनेल
    .show-number = कार्यक्षेत्र संख्या दिखाएँ
    .show-name = कार्यक्षेत्र का नाम दिखाएँ
workspaces-orientation = कार्यक्षेत्र की दिशा
    .vertical = ऊर्ध्वाधर
    .horizontal = क्षैतिज
hot-corner = हॉट कॉर्नर
    .top-left-corner = कार्यक्षेत्रों के लिए ऊपरी बाएँ कोने में घुमाएँ

## Displays

-requires-restart = पुनः आरंभ आवश्यक
color = रंग
    .depth = रंग की गहराई
    .profile = रंग प्रोफ़ाइल
    .sidebar = रंग प्रोफ़ाइल
    .temperature = रंग का तापमान
display = प्रदर्शन
    .desc = प्रदर्शन प्रबंधित करें, ग्राफिक्स परिवर्तन और रात की रोशनी
    .arrangement = प्रदर्शन व्यवस्था
    .arrangement-desc = पुनर्व्यवस्थित करने के लिए प्रदर्शन खींचें।
    .enable = प्रदर्शन को सक्रिय करें
    .external = { $size } { $output } बाहरी प्रदर्शन
    .laptop = { $size } लैपटॉप प्रदर्शन
    .options = प्रदर्शन विकल्प
    .refresh-rate = ताज़ा दर
    .resolution = संकल्प
    .scale = आकार
mirroring = परावर्तन
    .id = परावर्तित { $id }
    .dont = परावर्तित न करें
    .mirror = परावर्तित करें { $display }
    .project =
        { $display } पर प्रक्षिप्त करें { $display ->
            [all] सभी प्रदर्शन
           *[other] { $display }
        }
    .project-count =
        { $count } अन्य { $count ->
            [1] प्रदर्शन
           *[other] प्रदर्शन
        }
night-light = रात की रोशनी
    .auto = स्वचालित (सूर्यास्त से सूर्योदय तक)
    .desc = गर्म रंगों से नीली रोशनी को कम करें।
orientation = अभिविन्यास
    .standard = मानक
    .rotate-90 = 90 डिग्री पर घुमाएँ
    .rotate-180 = 180 डिग्री पर घुमाएँ
    .rotate-270 = 270 डिग्री पर घुमाएँ
scheduling = अनुसूची
    .manual = मैन्युअल अनुसूची
dialog = संवाद
    .title = क्या आप इन प्रदर्शन सेटिंग्स को बनाए रखेंगे?
    .keep-changes = परिवर्तन बनाए रखें
    .change-prompt = सेटिंग में परिवर्तन { $time } सेकंड में स्वचालित रूप से वापस आ जाएगा।
    .revert-settings = सेटिंग्स वापस करें
legacy-app-scaling = X11 विंडो सिस्टम एप्लिकेशन स्केलिंग
    .scaled-by-system = सभी X11 एप्लिकेशनों को स्केल करें
    .system-description = X11 एप्लिकेशन HiDPI स्क्रीन पर धुंधले दिखते हैं।
    .scaled-natively = स्थानीय संकल्प पर X11 एप्लिकेशनों को स्केल करें
    .native-description = HiDPI प्रदर्शन के उपयोग के दौरान स्केलिंग का समर्थन न करने वाले X11 एप्लिकेशन छोटे हो जाते हैं। खेलों के लिए पूर्ण मॉनिटर संकल्प का उपयोग करने के लिए सक्रिय करें।

## Sound

sound = ध्वनि
    .desc = लागू नहीं
sound-output = आउटपुट
    .volume = आउटपुट ध्वनि
    .device = आउटपुट उपकरण
    .level = आउटपुट स्तर
    .config = कॉन्फ़िगरेशन
    .balance = संतुलन
sound-input = इनपुट
    .volume = इनपुट ध्वनि
    .device = इनपुट उपकरण
    .level = इनपुट स्तर
sound-alerts = अलार्म
    .volume = अलार्म ध्वनि
    .sound = अलार्म ध्वनि
sound-applications = अनुप्रयोग
    .desc = अनुप्रयोग ध्वनियाँ और सेटिंग्स
profile = प्रोफ़ाइल

## Power

power = शक्ति और बैटरी
    .desc = शक्ति सेटिंग्स प्रबंधित करें
battery = बैटरी
    .minute =
        { $value } { $value ->
            [one] मिनट
           *[other] मिनट
        }
    .hour =
        { $value } { $value ->
            [one] घंटा
           *[other] घंटे
        }
    .day =
        { $value } { $value ->
            [one] दिन
           *[other] दिन
        }
    .less-than-minute = एक मिनट से कम
    .and = और
    .remaining-time =
        { $time } तक { $action ->
            [full] पूर्ण
           *[other] खाली
        }
connected-devices = कनेक्टेड डिवाइस
    .unknown = अज्ञात डिवाइस
power-mode = पावर मोड
    .battery = विस्तारित बैटरी जीवन
    .battery-desc = कम ऊर्जा उपयोग और शांत कार्यक्षमता।
    .balanced = संतुलन
    .balanced-desc = शांत कार्यक्षमता और मध्यम ऊर्जा उपयोग।
    .performance = उच्च कार्यक्षमता
    .performance-desc = उत्कृष्ट प्रदर्शन और उच्च ऊर्जा उपयोग।
    .no-backend = कोई बैकएंड नहीं मिला। system76-power या power-profiles-daemon स्थापित करें।

## Input

acceleration-desc = गति के आधार पर ट्रैकिंग संवेदनशीलता स्वचालित रूप से समायोजित होती है।
disable-while-typing = टाइप करते समय निष्क्रिय करें
input-devices = इनपुट डिवाइस
    .desc = इनपुट डिवाइस
primary-button = प्राथमिक बटन
    .desc = भौतिक बटनों का क्रम सेट करता है।
    .left = बायां
    .right = दायां
scrolling = स्क्रॉलिंग
    .two-finger = दो अंगुलियों से स्क्रॉल करें
    .edge = एक अंगुली से किनारे पर स्क्रॉल करें
    .speed = स्क्रॉलिंग गति
    .natural = प्राकृतिक स्क्रॉलिंग
    .natural-desc = सामग्री को दृश्य के विपरीत स्क्रॉल करें

## Input: Keyboard

slow = धीमा
fast = तेज
short = छोटा
long = लंबा
keyboard = कीबोर्ड
    .desc = इनपुट स्रोत, परिवर्तन, विशेष वर्णों का इनपुट, शॉर्टकट।
keyboard-sources = इनपुट स्रोत
    .desc = इनपुट स्रोतों को Super+Space दबाकर बदला जा सकता है। इसे कीबोर्ड शॉर्टकट में कस्टमाइज़ किया जा सकता है।
    .move-up = ऊपर जाएं
    .move-down = नीचे जाएं
    .settings = सेटिंग्स
    .view-layout = कीबोर्ड लेआउट देखें
    .remove = इस स्रोत को हटाएं
    .add = इनपुट स्रोत जोड़ें
keyboard-special-char = विशेष वर्णों का इनपुट
    .alternate = वैकल्पिक वर्ण कुंजी
    .compose = संयोजन कुंजी
    .caps = कैप्स लॉक कुंजी
keyboard-typing-assist = टाइपिंग
    .repeat-rate = पुनरावृत्ति दर
    .repeat-delay = पुनरावृत्ति विलंब
added = जोड़ा गया
type-to-search = खोजने के लिए टाइप करें...
show-extended-input-sources = विस्तारित इनपुट स्रोत दिखाएं

## Input: Keyboard: Shortcuts

keyboard-shortcuts = कीबोर्ड शॉर्टकट
    .desc = शॉर्टकट देखें और कस्टमाइज़ करें
add-keybinding = कीबाइंडिंग जोड़ें
cancel = रद्द करें
command = कमांड
custom = कस्टम
debug = डिबग
disabled = निष्क्रिय
migrate-workspace-prev = कार्यक्षेत्र को पिछले आउटपुट में माइग्रेट करें
migrate-workspace-next = कार्यक्षेत्र को अगले आउटपुट में माइग्रेट करें
migrate-workspace =
    कार्यक्षेत्र को { $direction ->
       *[down] नीचे
        [left] बायां
        [right] दायां
        [up] ऊपर
    }
navigate = नेविगेट करें
replace = बदलें
shortcut-name = शॉर्टकट नाम
system-controls = सिस्टम नियंत्रण
terminate = समाप्त करें
toggle-stacking = विंडो स्टैकिंग टॉगल करें
type-key-combination = कुंजी संयोजन टाइप करें
custom-shortcuts = कस्टम शॉर्टकट
    .add = शॉर्टकट जोड़ें
    .context = कस्टम शॉर्टकट जोड़ें
    .none = कोई कस्टम शॉर्टकट नहीं है
modified = { $count } संशोधित
nav-shortcuts = नेविगेशन
    .prev-output = पिछले आउटपुट पर ध्यान केंद्रित करें
    .next-output = अगले आउटपुट पर ध्यान केंद्रित करें
    .last-workspace = अंतिम कार्यक्षेत्र पर ध्यान केंद्रित करें
    .prev-workspace = पिछले कार्यक्षेत्र पर ध्यान केंद्रित करें
    .next-workspace = अगले कार्यक्षेत्र पर ध्यान केंद्रित करें
    .focus =
        विंडो { $direction ->
           *[down] नीचे
            [in] अंदर
            [left] बायां
            [out] बाहर
            [right] दायां
            [up] ऊपर
        } पर ध्यान केंद्रित करें
    .output =
        आउटपुट पर स्विच करें { $direction ->
           *[down] नीचे
            [left] बायां
            [right] दायां
            [up] ऊपर
        }
    .workspace = कार्यक्षेत्र पर स्विच करें { $num }
manage-windows = विंडो प्रबंधित करें
    .close = विंडो बंद करें
    .maximize = विंडो अधिकतम करें
    .minimize = विंडो न्यूनतम करें
    .resize-inwards = विंडो को अंदर की ओर आकार दें
    .resize-outwards = विंडो को बाहर की ओर आकार दें
    .toggle-sticky = चिपकने वाली विंडो टॉगल करें
move-windows = विंडो को स्थानांतरित करें
    .direction =
        विंडो { $direction ->
           *[down] नीचे
            [left] बाएँ
            [right] दाएँ
            [up] ऊपर
        } पर ले जाएं
    .display =
        विंडो को एक मॉनिटर { $direction ->
           *[down] नीचे
            [left] बाएँ
            [right] दाएँ
            [up] ऊपर
        } पर ले जाएं
    .workspace =
        विंडो को एक कार्यक्षेत्र { $direction ->
           *[below] नीचे
            [left] बाएँ
            [right] दाएँ
            [above] ऊपर
        } पर ले जाएं
    .workspace-num = विंडो को कार्यक्षेत्र { $num } पर ले जाएं
    .prev-workspace = विंडो को पिछले कार्यक्षेत्र पर ले जाएं
    .next-workspace = विंडो को अगले कार्यक्षेत्र पर ले जाएं
    .last-workspace = विंडो को अंतिम कार्यक्षेत्र पर ले जाएं
    .next-display = विंडो को अगले डिस्प्ले पर ले जाएं
    .prev-display = विंडो को पिछले डिस्प्ले पर ले जाएं
    .send-to-prev-workspace = विंडो को पिछले कार्यक्षेत्र में भेजें
    .send-to-next-workspace = विंडो को अगले कार्यक्षेत्र में भेजें
system-shortcut = सिस्टम
    .app-library = ऐप पुस्तकालय खोलें
    .brightness-down = डिस्प्ले की चमक कम करें
    .brightness-up = डिस्प्ले की चमक बढ़ाएं
    .home-folder = होम फ़ोल्डर खोलें
    .keyboard-brightness-down = कीबोर्ड की चमक कम करें
    .keyboard-brightness-up = कीबोर्ड की चमक बढ़ाएं
    .launcher = लॉन्चर खोलें
    .lock-screen = स्क्रीन लॉक करें
    .mute = ऑडियो आउटपुट म्यूट करें
    .mute-mic = माइक्रोफोन इनपुट म्यूट करें
    .play-pause = प्ले/पॉज़ करें
    .play-next = अगले ट्रैक पर जाएं
    .play-prev = पिछले ट्रैक पर जाएं
    .screenshot = स्क्रीनशॉट लें
    .terminal = टर्मिनल खोलें
    .volume-lower = ऑडियो आउटपुट वॉल्यूम कम करें
    .volume-raise = ऑडियो आउटपुट वॉल्यूम बढ़ाएं
    .web-browser = वेब ब्राउज़र खोलें
    .window-switcher = खुली विंडो के बीच स्विच करें
    .workspace-overview = कार्यक्षेत्र का अवलोकन खोलें
window-tiling = विंडो टाइलिंग
    .horizontal = क्षैतिज दिशा में संरेखित करें
    .vertical = ऊर्ध्वाधर दिशा में संरेखित करें
    .swap-window = विंडो स्वैप करें
    .toggle-tiling = विंडो टाइलिंग टॉगल करें
    .toggle-stacking = विंडो स्टैकिंग टॉगल करें
    .toggle-floating = विंडो फ्लोटिंग टॉगल करें
    .toggle-orientation = दिशा बदलें
replace-shortcut-dialog = शॉर्टकट बदलें?
    .desc = { $shortcut } का उपयोग { $name } कर रहा है। यदि आप इसे बदलते हैं, तो { $name } अक्षम हो जाएगा।

## Input: Mouse

mouse = माउस
    .desc = माउस की गति, त्वरण, प्राकृतिक स्क्रॉलिंग।
    .speed = माउस की गति
    .acceleration = माउस त्वरण सक्षम करें

## Input: Touchpad

click-behavior = क्लिक व्यवहार
    .click-finger = दो उंगलियों से सेकेंडरी क्लिक और तीन उंगलियों से मध्य क्लिक
    .button-areas = बाएँ और नीचे मध्य क्लिक के लिए नीचे दाईं ओर
pinch-to-zoom = पिंच-टू-ज़ूम
    .desc = दो उंगलियों का उपयोग करके सामग्री को ज़ूम करें, उन एप्लिकेशन के लिए जो ज़ूम का समर्थन करते हैं।
tap-to-click = टैप टू क्लिक
    .desc = प्राथमिक क्लिक के लिए एक उंगली का टैप, सेकेंडरी क्लिक के लिए दो उंगलियों का टैप, और मध्य क्लिक के लिए तीन उंगलियों का टैप सक्षम करता है।
touchpad = टचपैड
    .acceleration = टचपैड त्वरण सक्षम करें
    .desc = टचपैड की गति, क्लिक विकल्प, इशारे।
    .speed = टचपैड की गति

## Input: Gestures

gestures = इशारे
    .four-finger-down = चार उंगलियों की नीचे की दिशा में इशारा
    .four-finger-left = चार उंगलियों की बाईं दिशा में इशारा
    .four-finger-right = चार उंगलियों की दाईं दिशा में इशारा
    .four-finger-up = चार उंगलियों की ऊपर की दिशा में इशारा
    .three-finger-any = तीन उंगलियों की किसी भी दिशा में इशारा
switch-workspaces = कार्यक्षेत्रों को बदलें
    .horizontal = चार उंगलियों का बायाँ/दायाँ इशारा
    .vertical = चार उंगलियों का ऊपर/नीचे इशारा
switch-between-windows = विंडो के बीच स्विच करें
open-application-library = ऐप लाइब्रेरी खोलें
open-workspaces-view = कार्यक्षेत्र का अवलोकन खोलें

## Time & Language

time = समय और भाषा
    .desc = लागू नहीं
time-date = दिनांक और समय
    .desc = समय क्षेत्र, स्वचालित घड़ी सेटिंग्स, और कुछ समय प्रारूप।
    .auto = स्वचालित रूप से समायोजित करें
    .auto-ntp = समय क्षेत्र को समायोजित करने पर दिनांक और समय स्वचालित रूप से अपडेट होता है।
time-zone = समय क्षेत्र
    .auto = स्वचालित समय क्षेत्र
    .auto-info = स्थान सेवा और इंटरनेट कनेक्शन की आवश्यकता है
time-format = दिनांक और समय प्रारूप
    .twenty-four = 24-घंटे का समय
    .show-seconds = सेकंड दिखाएं
    .first = सप्ताह का पहला दिन
    .show-date = ऊपरी पैनल में दिनांक दिखाएं
    .friday = शुक्रवार
    .saturday = शनिवार
    .sunday = रविवार
    .monday = सोमवार
time-region = क्षेत्र और भाषा
    .desc = आपके क्षेत्र के आधार पर दिनांक, समय, और संख्या प्रारूपित करें

## System

system = सिस्टम और खाते

## System: About

about = बारे में
    .desc = डिवाइस नाम, हार्डवेयर जानकारी, ऑपरेटिंग सिस्टम डिफ़ॉल्ट।
about-device = डिवाइस का नाम
    .desc = यह नाम अन्य नेटवर्क या ब्लूटूथ उपकरणों के लिए दिखाई देता है।
about-hardware = हार्डवेयर
    .model = हार्डवेयर मॉडल
    .memory = मेमोरी
    .processor = प्रोसेसर
    .graphics = ग्राफिक्स
    .disk-capacity = डिस्क क्षमता
about-os = ऑपरेटिंग सिस्टम
    .os = ऑपरेटिंग सिस्टम
    .os-architecture = ऑपरेटिंग सिस्टम आर्किटेक्चर
    .desktop-environment = डेस्कटॉप वातावरण
    .windowing-system = विंडो नियंत्रण प्रणाली
about-related = संबंधित सेटिंग्स
    .support = सहायता प्राप्त करें

## System: Firmware

firmware = फर्मवेयर
    .desc = फर्मवेयर विवरण।

## System: Users

users = उपयोगकर्ता
    .desc = प्रमाणीकरण और उपयोगकर्ता खाते।
password-confirm = पासवर्ड की पुष्टि करें
identity = पहचान
activate = सक्रिय करें
enable = सक्षम करें
