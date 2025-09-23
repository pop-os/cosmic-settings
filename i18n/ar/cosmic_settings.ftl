app = إعدادات COSMIC
dbus-connection-error = تعذَّر الاتصال بـDBus
ok = حسنًا
unknown = مجهول
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] سلكي
        [wifi] واي فاي
        [vpn] شبكة خاصة افتراضية
       *[other] مجهول
    } الاتصالات ولاحات الاتصالات.
add-network = أضِف شبكة
    .profile = أضِف ملف تعريف
add-vpn = أضِف شبكة خاصة افتراضية
airplane-on = وضع الطائرة مفعَّل.
cable-unplugged = الكبل مفصول
connect = اتصل
connected = متَّصل
connecting = يتَّصل…
disconnect = اقطع الاتِّصال
forget = انسَ
known-networks = الشبكات المعروفة
network-and-wireless = الشبكة واللاسلكي
no-networks = لم يُعثر على شبكات.
no-vpn = لا اتصالات شبكات خاصة افتراضية متوفرة.
password = كلمة السر
password-confirm = أكِّد كلمة السر
remove = أزِل
settings = الإعدادات
username = اسم المستخدم
visible-networks = الشبكات المرئية
identity = الهوية
auth-dialog = الاستيثاق مطلوب
    .vpn-description = أدخِل اسم المستخدم وكلمة السر المطلوبة من قبل خدمة الشبكة الخاصة الافتراضية.
    .wifi-description = أدخِل كلمة السر أو مفتاح التعمية. يمكنك أيضًا الاتِّصال عبر ضغط زر “WPS” على الموجِّه.
forget-dialog = أنسَ شبكة واي فاي هذه؟
    .description = سيتعين إدخال كلمة السر لاستخدامها مستقبلًا.
network-device-state =
    .activated = متَّصل
    .config = يتَّصل
    .deactivating = يقطع الاتِّصال
    .disconnected = قُطِع الاتِّصال
    .failed = تعذَّر الاتِّصال
    .ip-check = يتحقق من الاتِّصال
    .ip-config = يطلُب معلومات التوجيه والتراسل الفوريّ
    .need-auth = يتعين الاستيثاق
    .prepare = يحضِّر للاتصال
    .secondaries = ينتظر الاتِّصال الثانوي
    .unavailable = غير متاح
    .unknown = حالة مجهولة
    .unmanaged = غير مُدار
    .unplugged = الكبل مفصول
remove-connection-dialog = أزيلُ ملف تعريف الاتِّصال؟
    .vpn-description = سيتعين إدخال كلمة السر لاستخدام هذه الشبكة مستقبلًا.
    .wired-description = ستتعين إعادة إنشاء ملف التعريف هذا لاستخدامه مستقبلًا.
vpn = شبكة خاصة افتراضية
    .connections = اتصالات الشبكات الخاصة الافتراضية
    .error = تعذّّرت إضافة ضبط الشبكة الخاصة الافتراضية
    .remove = أزِل ملف تعريف الاتِّصال
    .select-file = اختر ملف ضبط الشبكة الخاصة الافتراضية
vpn-error = خطأ في الشبكة الخاصة الافتراضية
    .config = تعذَّرت إضافة ضبط الشبكة الخاصة الافتراضية
    .connect = تعذَّر الاتِّصال بالشبكة الخاصة الافتراضية
    .connection-editor = فشل محرر الاتِّصال
    .connection-settings = تعذَّر جلب إعدادات الاتِّصالات النشطة
    .updating-state = تعذَّر تحديث حالة مدير الشبكة
    .wireguard-config-path = مسار ملف غير صالح لضبط واير غارد
    .wireguard-config-path-desc = ينبغي أن يكون الملف المحدد على نظام الملفات المحلي.
    .wireguard-device = تعذَّر إنشاء جهاز واير غارد
    .with-password =
        تعذَّر تعيين الشبكة الخاصة الافتراضية { $field ->
           *[username] اسم المستخدم
            [password] كلمة السر
            [password-flags] خيارات كلمة السر
        } باستخدام nmcli
wired = سلكي
    .adapter = المحوِّل السلكي { $id }
    .connections = الاتصالات السلكية
    .devices = الأجهزة السلكة
    .remove = أزِل ملف تعريف الاتِّصال
wifi = واي فاي
    .adapter = محوِّل واي فاي { $id }
    .forget = انسَ هذه الشبكة
wireguard-dialog = أضِف جهاز واير غارد
    .description = اختر اسم جهاز لضبط واير غارد.

## Networking: Online Accounts

online-accounts = حسابات الإنترنت
    .desc = أضِف الحسابات، وIMAP، وSMTP، وولوجات العمل

# Bluetooth

activate = نشّط
confirm = أكِّد
enable = فعِّل
bluetooth = بلوتوث
    .desc = أدر أجهزة بلوتوث
    .status = سيظهر النظام باسم { $aliases } حين تُفتح إعدادات بلوتوث.
    .connected = متَّصل
    .connecting = يتَّصل
    .disconnecting = يقطع الاتِّصال
    .connect = اتَّصِل
    .disconnect = اقطع الاتِّصال
    .forget = انسَ
    .dbus-error = حدث خطأ أثناء التعامل مع DBus: { $why }
    .disabled = خدمة البلوتوث معطّلة
    .inactive = خدمة البلوتوث غير نشطة
    .unknown = تعذر تنشيط خدمة البلوتوث. هل BlueZ مثبت؟
bluetooth-paired = الأجهزة المتَّصَلة مسبقًا
    .connect = اتَّصِل
    .battery = البطارية { $percentage }٪
bluetooth-confirm-pin = أكِّد رمز بلوتوث
    .description = تحقق بأن الرمز التالي يطابق المعروض على { $device }
bluetooth-available = الأجهزة القريبة
bluetooth-adapters = محوِّلات بلوتوث

## Accessibility

accessibility = الإتاحة
    .vision = الرؤية
    .on = مفعّل
    .off = معطّل
    .unavailable = غير متوفّر
    .screen-reader = قارئ الشاشة
    .high-contrast = وضع تباين عالٍ
    .invert-colors = اعكس الألوان
    .color-filters = تصفيات الألوان
hearing = السمع
    .mono = شغَّل الصوت الاستريو كأحادي
default = المبدئي
magnifier = المكبّر
    .controls =
        أو استخدم هذه الاختصارات: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } للتقريب,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } للتبعيد,
        }
        Super + التمرير بعجلة الفأرة
    .scroll_controls = فعِّل التكبير بالفأرة أو لوحة اللمس مع Super + التمرير
    .show_overlay = أظهر تراكب المكبّر
    .increment = زيادة التكبير
    .signin = بدء المكبّر عند تسجيل الدخول
    .applet = تبديل المكبر تشغيل/إيقاف في البُريمجات على اللوحة
    .movement = عرض مكبّر يتحرك
    .continuous = بشكل مستمر مع المؤشر
    .onedge = عند وصول المؤشر إلى الحافة
    .centered = لإبقاء المؤشر في الوسط
color-filter = نوع تصفية الألوان
    .unknown = تصفية غير معروفة نشطة
    .greyscale = تدرّج رمادي
    .deuteranopia = أخضر/أحمر (ضعف في الأخضر، الديوترانوبيا)
    .protanopia = أحمر/أخضر (ضعف في الأحمر، البروتانوبيا)
    .tritanopia = أزرق/أصفر (ضعف في الأزرق، التريتانوبيا)

## Desktop

desktop = سطح المكتب

## Desktop: Wallpaper

wallpaper = خلفية الشاشة
    .change = يغير الصورة كل
    .desc = صور الخلفية، والألوان، وخيارات عرض الشرائح.
    .fit = ملائمة الخلفية
    .folder-dialog = اختر مجلد الخلفية
    .image-dialog = اختر صورة الخفية
    .plural = الخلفيات
    .same = اعرض نفس الخلفية على كافة الشاشات
    .slide = عرض الشرائح
add-color = أضِف لونًا
add-image = أضِف صورة
all-displays = جميع الشاشات
colors = الألوان
dialog-add = أضِف
fill = مملوءة
fit-to-screen = ملائمة للشاشة
open-new-folder = افتح مجلدًا جديدًا
recent-folders = المجلدات الأخيرة
x-minutes =
    { $number ->
        [0] أقل من دقيقة
        [1] دقيقة واحدة
        [2] دقيقتان
        [few] { $number } دقائق
        [many] { $number } دقيقة
       *[other] { $number } دقيقة
    }
x-hours =
    { $number ->
        [0] أقل من دقيقة
        [1] دقيقة واحدة
        [2] دقيقتان
        [few] { $number } دقائق
        [many] { $number } دقيقة
       *[other] { $number } دقيقة
    }

## Desktop: Appearance

appearance = المظهر
    .desc = لون التمييز والسمات.
accent-color = لون التمييز
app-background = خلفية النافذة أو التطبيق
auto = آلي
close = أغلِق
color-picker = مُنْتَقِي اللون
copied-to-clipboard = نُسِخ إلى الحافظة
copy-to-clipboard = انسِخ إلى الحافظة
dark = داكن
export = صدِّر
hex = ست عشري
import = استورد
light = فاتح
mode-and-colors = الوضع والألوان
recent-colors = الألوان الأخيرة
reset-to-default = صفِّر إلى المبدئيات
rgb = ح‌خ‌ز
window-hint-accent = لون تلميح النافذة النشطة
window-hint-accent-toggle = استخدام تمييز اللون للسمة كنافذة تلميح نشطة
auto-switch = التبديل تلقائيًا بين الوضعين الفاتح والداكن
    .sunrise = يبدِّل إلى الوضع الفاتح عند شروق الشمس
    .sunset = يبدِّل إلى الوضع الداكن عند غروب الشمس
    .next-sunrise = يبدِّل إلى الوضع الفاتح عند شروق الشمس المقبل
    .next-sunset = يبدِّل إلى الوضع الداكن عند غروب الشمس المقبل
container-background = خلفية الحاوية
    .desc-detail = لون خلفية الحاوية يستخدم لشريط التنقل الجانبي، والدرج الجانبي، والحواريات، والودجات المشابهة. يستمد اللون آليًا من التطبيق أو نافذة الخلفية مبدئيًا.
    .reset = صفِّر إلى الآلي
    .desc = يستخدم اللون الأساسي للحاوية لشريط التنقل الجانبي، والدرج الجانبي، والحواريات، والودجات المشابهة.
control-tint = صيغة مكون التحكم
    .desc = يستعمل لخلفيات الأزرار القياسية، ومدخلات البحث، ومدخلات النص، والمكوِّنات المشابهة.
frosted = تأثير الزجاج المسنفر على واجهة النظام
    .desc = يطبِّق تمويه الخلفية للشريط، والبُرَيْمجات، والمطلق، ومكتبة التطبيقات.
enable-export = طبِّق هذه السمة على تطبيقات جنوم.
    .desc = لا تدعم كل عُدَد الأدوات التبديل الآلي. بقية التطبيقات غير المرتبطة بـCOSMIC ستحتاج لإعادة التشغيل بعد تغيير السمة.
icon-theme = سمة الأيقونات
    .desc = يطبِّق مجموعة مختلفة من الأيقونات للتطبيقات.
text-tint = صيغة نص الواجهة
    .desc = اللون المستخدم لاشتقاق ألوان النصوص في الواجهة بحيث تحقق تباينًا كافيًا على الأسطح المختلفة.
style = الأسلوب
    .round = مدوَّر
    .slightly-round = مدوَّر نسبيًا
    .square = مربع
interface-density = كثافة الواجهة
    .comfortable = مريحة
    .compact = مدمجة
    .spacious = واسعة
window-management-appearance = إدارة النوافذ
    .active-hint = حجم تلميح النافذة النشطة
    .gaps = الفراغات حول النوافذ المبلَّطة

### Experimental

experimental-settings = الإعدادات التجريبية
icons-and-toolkit = سمات الأيقونات وعُدَد الأدوات
interface-font = خط النظام
monospace-font = الخط ثابت العرض

## Desktop: Notifications

notifications = التنبيهات
    .desc = عدم الإزعاج، وتنبيهات شاشة القفل، والإعداد حسب التطبيق.

## Desktop: Panel

panel = الشريط
    .desc = شريط النظام الأساسي للقوائم والبُريمجات.
add = أضِف
add-applet = أضِف بُريمج
all = الكل
applets = البُريمجات
center-segment = قطعة المركز
drop-here = ألقِ البُريمجات هنا
end-segment = قطعة النهاية
large = كبير
no-applets-found = لم يُعثر على بُريمجات…
panel-bottom = أدنى
panel-left = يسار
panel-right = يمين
panel-top = أعلى
search-applets = ابحث البُريمجات…
small = صغير
start-segment = قطعة البداية
panel-appearance = المظهر
    .match = مطابقة سطح المكتب
    .light = فاتح
    .dark = داكن
panel-behavior-and-position = السلوك والمواضع
    .autohide = أخفِ الشريط آليًا
    .dock-autohide = أخفِ المرسى آليًا
    .position = الموضع على الشاشة
    .display = اعرض على الشاشة
panel-style = الأسلوب
    .anchor-gap = فراغ بين الشريط وحواف الشاشة
    .dock-anchor-gap = فراغ بين المرسى وحواف الشاشة
    .extend = تمديد الشريط إلى حواف الشاشة
    .dock-extend = تمديد المرسى إلى حواف الشاشة
    .appearance = المظهر
    .size = الحجم
    .background-opacity = عتمة الخلفية
panel-applets = الضبط
    .dock-desc = اضبط بُريمجات المرسى
    .desc = اضبط بُريمجات الشريط
panel-missing = ضبط الشريط مفقود
    .desc = فُقِد ضبط الشريط لاستخدام ضبط مخصَّص أو لأنه معطوب.
    .fix = صفِّر إلى المبدئيات

## Desktop: Dock

dock = المرسى
    .desc = شريط اختياري للتطبيقات وبُريمجات.

## Desktop: Window management

window-management = إدارة النوافذ
    .desc = إجراء المفتاح Super وخيارات تحكم النوافذ، وخيارات تبليط نوافذ إضافية.
super-key = إجراء المفتاح Super
    .launcher = فتح المطلق
    .workspaces = فتح مساحات العمل
    .applications = فتح التطبيقات
    .disable = معطَّل
edge-gravity = تنجذب النوافذ العائمة إلى الحواف القريبة
window-controls = تحكمات النوافذ
    .maximize = أظهر زر التكبير
    .minimize = أظهر زر التصغير
    .active-window-hint = أظهر تلميح النافذة النشطة
focus-navigation = بؤرة التنقل
    .focus-follows-cursor = البؤرة تتبع المؤشر
    .focus-follows-cursor-delay = تأخير اتباع البؤرة للمؤشر بالملي ثانية
    .cursor-follows-focus = المؤشر يتبع البؤرة

## Desktop: Workspaces

workspaces = مساحات العمل
    .desc = سلوك واتجاه مساحات العمل.
workspaces-behavior = سلوك مساحة العمل
    .dynamic = مساحات عمل حركيَّة
    .dynamic-desc = يُزيل مساحات العمل الفارغة آليًا.
    .fixed = عدد محدّد من مساحات العمل
    .fixed-desc = يُضيف أو يُزيل مساحات العمل للنظرة العامة.
workspaces-multi-behavior = سلوك المرقابات المتعدد
    .span = مساحات العمل تمتد عبر الشاشات
    .separate = لكل شاشة مساحات عمل مختلفة
workspaces-overview-thumbnails = مُصغَّرات منظور مساحة العمل العام
    .show-number = أظهر رقم مساحة العمل
    .show-name = أظهر اسم مساحة العمل
workspaces-orientation = اتجاه مساحات العمل
    .vertical = عمودي
    .horizontal = أفقي
hot-corner = الزاوية الساخنة
    .top-left-corner = فعِّل الزاوية الساخنة أعلى اليسار لمساحات العمل

## Displays

-requires-restart = يتطلب إعادة التشغيل
color = اللون
    .depth = عمق اللون
    .profile = ملف تعريف اللون
    .sidebar = ملفات تعريف اللون
    .temperature = حرارة اللون
display = الشاشات
    .desc = أدر الشاشات والإضاءة الليلية
    .arrangement = ترتيبة الشاشة
    .arrangement-desc = اسحب الشاشات لإعادة ترتيبها.
    .enable = فعِّل الشاشة
    .external = { $size } { $output } شاشة خارجية
    .laptop = { $size } شاشة حاسب محمول
    .options = خيارات العرض
    .refresh-rate = معدل التحديث
    .resolution = الميز
    .scale = المقياس
    .additional-scale-options = خيارات مقياس إضافية
mirroring = المرآوية
    .id = يعكس { $id }
    .dont = لا تعكس
    .mirror = اعكس { $display }
    .project =
        اعرض إلى { $display ->
            [all] كل الشاشات
           *[other] { $display }
        }
    .project-count =
        يعرض إلى { $count ->
            [0] لا شاشات
            [1] شاشة واحدة أخرى
            [2] شاشتين أخرتين
            [few] { $count } شاشات أخرى
            [many] { $count } شاشة أخرى
           *[other] { $count } شاشة أخرى
        }
night-light = الإضاءة الليلية
    .auto = آلي (من شروق الشمس إلى مغربها)
    .desc = يخفف الضوء الأزرق بألوان أكثر دفئًا.
orientation = الاتِّجاه
    .standard = قياسي
    .rotate-90 = أدِر 90°
    .rotate-180 = أدِر 180°
    .rotate-270 = أدِر 270°
vrr = معدّل التحديث المتغير
    .enabled = مفعَّل
    .force = دائمًا
    .auto = تلقائي
    .disabled = معطَّل
scheduling = الجدولة
    .manual = جدولة يدوية
dialog = الحواري
    .title = أبقي هذه الإعدادات؟
    .keep-changes = أبقِ الإعدادات
    .change-prompt =
        سترتد الإعدادات خلال { $time ->
            [0] أقل من ثانية
            [1] ثانية واحدة
            [2] ثانيتين
            [few] { $time } ثواني
            [many] { $time } ثانية
           *[other] { $time } ثانية
        }.
    .revert-settings = استرد الإعدادات

## Sound

sound = الصوت
    .desc = غير متوفِّر
sound-output = الإخراج
    .volume = مستوى صوت المخرج
    .device = جهاز الإخراج
    .level = مستوى الإخراج
    .config = الضبط
    .balance = التوازن
    .left = يسار
    .right = يمين
sound-input = الإدخال
    .volume = مستوى صوت المدخل
    .device = جهاز الإدخال
    .level = مستوى الإدخال
amplification = تضخيم
    .desc = يسمح برفع مستوى الصوت إلى 150٪.
sound-alerts = الإنذارات
    .volume = مستوى صوت الإنذارات
    .sound = صوت الإنذارات
sound-applications = التطبيقات
    .desc = مستوى صوت التطبيقات والإعدادات
profile = ملف التعريف

## Power

power = الطاقة والبطارية
    .desc = أدر إعدادات الطاقة
battery = البطارية
    .minute =
        { $value ->
            [zero] أقل من دقيقة
            [one] دقيقة واحدة
            [two] دقيقتان
            [few] { $value } دقائق
            [many] { $value } دقيقة
           *[other] { $value } دقيقة
        }
    .hour =
        { $value ->
            [zero] أقل من ساعة
            [one] ساعة واحدة
            [two] ساعتان
            [few] { $value } ساعات
            [many] { $value } ساعة
           *[other] { $value } ساعة
        }
    .day =
        { $value ->
            [zero] أقل من يوم
            [one] يوم واحد
            [two] يومان
            [few] { $value } أيام
            [many] { $value } يومًا
           *[other] { $value } يوم
        }
    .less-than-minute = أقل من دقيقة
    .and = و
    .remaining-time =
        { $time } حتى { $action ->
            [full] تمتلئ
           *[other] تفرغ
        }
connected-devices = الأجهزة المتَّصلة
    .unknown = جهاز مجهول
power-mode = نمط الطاقة
    .battery = عمر بطارية ممتد
    .battery-desc = استهلاك طاقة مخفض وأداء صامت.
    .balanced = متوازن
    .balanced-desc = استهلاك طاقة متوازن وأداء هادئ.
    .performance = أداء عالي
    .performance-desc = قمة استهلاك الطاقة والأداء.
    .no-backend = لم يُعثَر على المنتهى الخلفي. ثبِّت system76-power أو power-profiles-daemon.
power-saving = خيارات توفير الطاقة
    .turn-off-screen-after = أوقِف تشغيل الشاشة بعد
    .auto-suspend = تعليق تلقائي
    .auto-suspend-ac = علِّق تلقائيًا عند التوصيل
    .auto-suspend-battery = علِّق تلقائيًا عند استخدام طاقة البطارية

## Input

acceleration-desc = يعيِّن حساسية التتبع آليًا استنادًا للسرعة.
disable-while-typing = عطِّل أثناء الكتابة
input-devices = أجهزة الإدخال
    .desc = أجهزة الإدخال
primary-button = الزر الأساسي
    .desc = يعيِّن ترتيب الأزرار الملموسة.
    .left = يسار
    .right = يمين
scrolling = الزلق
    .two-finger = الزلق باصبعين
    .edge = الزلق على الحافة بإصبع واحد
    .speed = سرعة الزلق
    .natural = الزلق الطبيعي
    .natural-desc = يزلِق المحتوى بدلًا من الرؤية

## Input: Keyboard

slow = بطيئة
fast = سريعة
short = قصيرة
long = طويلة
keyboard = لوحة المفاتيح
    .desc = مصادر الإدخال، والتبديل، وإدخال المحارف الخاصة، والاختصارات.
keyboard-sources = مصادر الإدخال
    .desc = يمكن تبديل مصادر الإدخال باستعمال تجميعة مفتاحي Super+المسافة. يمكن أن يخصص هذا من خلال إعدادات اختصارات لوحة المفاتيح.
    .move-up = حرِك لأعلى
    .move-down = حرِك لأدنى
    .settings = الإعدادات
    .view-layout = اعرض تخطيط لوحة المفاتيح
    .remove = أزِل
    .add = أضِف مصدر إدخال
keyboard-special-char = إدخال المحارف الخاصة
    .alternate = مفتاح المحارف الثانوية
    .compose = مفتاح التركيب
    .caps = مفتاح قفل الأحرف الكبيرة
keyboard-typing-assist = الكتابة
    .repeat-rate = معدّل التكرار
    .repeat-delay = تأخير التكرار
added = مُضاف
type-to-search = اكتب تبحث…
show-extended-input-sources = أظهر مصادر الإدخال الممتدة

## Input: Keyboard: Shortcuts

keyboard-shortcuts = اختصارات لوحة المفاتيح
    .desc = أظهِر وخصِص الاختصارات
add-another-keybinding = أضِف اختصارًا آخر
cancel = ألغِ
command = الأمر
custom = مخصَّص
debug = نقِّح
disabled = معطَّل
input-source-switch = بدِّل مصدر إدخال لغة لوحة المفاتيح
migrate-workspace-prev = انقل مساحة العمل للمخرج السابق
migrate-workspace-next = انقل مساحة العمل للمخرج التالي
migrate-workspace =
    نقل مساحة العمل للمخرج { $direction ->
       *[down] الأدنى
        [left] اليسار
        [right] اليمين
        [up] الأعلى
    }
navigate = التنقل
replace = استبدل
shortcut-name = اسم الاختصار
system-controls = تحكمات النظام
terminate = أنهِ
toggle-stacking = تبديل تكديس النوافذ
type-key-combination = اكتب تجميعة المفاتيح
custom-shortcuts = الاختصارات المخصَّصة
    .add = أضِف اختصارًا
    .context = أضِف اختصارًا مخصصًا
    .none = لا اختصارات مخصَّصة
modified = { $count } معدَّل
nav-shortcuts = التنقل
    .prev-output = التركيز على المخرج السابق
    .next-output = التركيز على المخرج التالي
    .last-workspace = التركيز على آخر مساحة عمل
    .prev-workspace = التركيز على مساحة العمل السابقة
    .next-workspace = التركيز على مساحة العمل التالية
    .focus =
        نافذة التركيز { $direction ->
           *[down] لأسفل
            [in] للداخل
            [left] لليسار
            [out] للخارج
            [right] لليمين
            [up] لأعلى
        }
    .output =
        التحويل إلى المخرج { $direction ->
           *[down] الأسفل
            [left] اليسار
            [right] اليمين
            [up] الأعلى
        }
    .workspace = التحويل إلى مساحة العمل رقم { $num }
manage-windows = أدر النوافذ
    .close = أغلق النافذة
    .maximize = كبّر النافذة
    .fullscreen = نافذة ملء الشاشة
    .minimize = صغّر النافذة
    .resize-inwards = حجّم النافذة للداخل
    .resize-outwards = حجّم النافذة للخارج
    .toggle-sticky = تبديل النافذة اللاصقة
move-windows = نقل النوافذ
    .direction =
        نقل النافذة { $direction ->
           *[down] لأسفل
            [left] لليسار
            [right] لليمين
            [up] لأعلى
        }
    .display =
        نقل النافذة باتجاه شاشة أخرى { $direction ->
           *[down] لأسفل
            [left] لليسار
            [right] لليمين
            [up] لأعلى
        }
    .workspace =
        نقل النافذة باتجاه مساحة عمل أخرى { $direction ->
           *[below] لأسفل
            [left] لليسار
            [right] لليمين
            [above] لأعلى
        }
    .workspace-num = نقل النافذة باتجاه مساحة العمل رقم { $num }
    .prev-workspace = نقل النافذة لمساحة العمل السابقة
    .next-workspace = نقل النافذة لمساحة العمل التالية
    .last-workspace = نقل النافذة لآخر مساحة عمل
    .next-display = نقل النافذة للشاشة التالية
    .prev-display = نقل النافذة للشاشة السابقة
    .send-to-prev-workspace = نقل النافذة لمساحة العمل السابقة
    .send-to-next-workspace = نقل النافذة لمساحة العمل التالية
system-shortcut = النظام
    .app-library = فتح مكتبة التطبيقات
    .brightness-down = تقليل سطوع الشاشة
    .brightness-up = زيادة سطوع الشاشة
    .home-folder = فتح مجلد المنزل
    .keyboard-brightness-down = تقليل سطوع لوحة المفاتيح
    .keyboard-brightness-up = زيادة سطوع لوح المفاتيح
    .launcher = فتح المطلق
    .log-out = الخروج
    .lock-screen = قفل الشاشة
    .mute = كتم مخرج الصوت
    .mute-mic = كتم مدخل الميكرفون
    .play-pause = التشغيل/الإيقاف
    .play-next = المقطوعة التالية
    .play-prev = المقطوعة السابقة
    .screenshot = التقاط لقطة شاشة
    .poweroff = أطفئ
    .terminal = فتح طرفيَّة
    .touchpad-toggle = بدّل لوحة اللمس
    .volume-lower = تقليل مستوى صوت المخرج
    .volume-raise = رفع مستوى صوت المخرج
    .web-browser = فتح متصفح وِب
    .window-switcher = التبديل بين النوافذ
    .window-switcher-previous = تبديل بين النوافذ المفتوحة معكوسًا
    .workspace-overview = فتح منظور مساحة العمل العام
window-tiling = تبليط النوافذ
    .horizontal = تعيين الاتجاه الأفقي
    .vertical = تعيين الاتجاه العمودي
    .swap-window = تبديل النافذة
    .toggle-tiling = تبديل تبليط النافذة
    .toggle-stacking = تبديل تكديس النافذة
    .toggle-floating = تبديل تعويم النافذة
    .toggle-orientation = تبديل الاتجاه
replace-shortcut-dialog = بدل الاختصار؟
    .desc = { $shortcut } مستخدم من قبل { $name }. إذا استبدِل، فسيعطَّل { $name }.
zoom-in = قرّب
zoom-out = بعّد

## Input: Mouse

mouse = الفأرة
    .desc = سرعة الفأرة، والتسارع، والزلق الطبيعي.
    .speed = سرعة الفأرة
    .acceleration = تفعيل تسارع الفأرة

## Input: Touchpad

click-behavior = سلوك النقر
    .click-finger = النقرة الثانوية بإصبعين والنقرة الوسطى بثلاث أصابع
    .button-areas = النقرة الثانوية أسفل اليمين والنقرة الوسطى في أسفل المنتصف
pinch-to-zoom = اقرِص تُكبِّر
    .desc = استخدام إصبعين لتحجيم محتوى التطبيقات التي تدعم ذلك.
tap-to-click = اضغط تنقر
    .desc = يفعِّل النقرة الأساسية بضغطة أصبع، و النقرة الثانوية بضغطة أصبعين، والنقرة الوسطى بضغط ثلاث أصابع.
touchpad = لوحة اللمس
    .acceleration = يفعِّل تسارع لوحة اللمس
    .desc = سرعة لوحة اللمس، وخيارات النقر، والإيماءات.
    .speed = سرعة لوحة اللمس

## Input: Gestures

gestures = الإيماءات
    .four-finger-down = تمرير أربع أصابع لأدنى
    .four-finger-left = تمرير أربع أصابع لليسار
    .four-finger-right = تمرير أربع أصابع لليمين
    .four-finger-up = تمرير أربع أصابع لأعلى
    .three-finger-any = تمرير ثلاث أصابع لأي اتجاه
switch-workspaces = تبديل مساحات العمل
    .horizontal = تمريرة أربع أصابع لليسار/اليمين
    .vertical = تمريرة أربع أصابع لأعلى/أدنى
switch-between-windows = التبديل بين النوافذ
open-application-library = فتح مكتبة التطبيقات
open-workspaces-view = فتح منظور مساحات العمل العام

## Time & Language

time = التوقيت واللغة
    .desc = غير متوفِّر
time-date = التاريخ والوقت
    .desc = المنطقة الزمنية، وإعدادات الساعة الآلية، وتنسيق الوقت.
    .auto = عيِّن آليًا
    .auto-ntp = سيحدَّث التاريخ والوقت آليًا عند تعيين المنطقة الزمنية.
time-zone = المنطقة الزمنية
    .auto = منطقة زمنية تلقائية
    .auto-info = يتطلب خدمات التموضع والوصول للإنترنت
time-format = تنسيق التاريخ والوقت
    .twenty-four = توقيت 24 ساعة
    .show-seconds = عرض الثواني
    .first = أول أيام الأسبوع
    .show-date = عرض التاريخ في بُريمج الوقت
    .friday = الجمعة
    .saturday = السبت
    .sunday = الأحد
    .monday = الاثنين
time-region = المنطقة واللغة
    .desc = نسق التواريخ، والتوقيت، والأرقام استنادًا لمنطقتك.
formatting = التنسيق
    .dates = التواريخ
    .time = الوقت
    .date-and-time = التاريخ والوقت
    .numbers = الأرقام
    .measurement = القياس
    .paper = ورقة
preferred-languages = اللغات المفضلة
    .desc = يحدد ترتيب اللُغات اللُغة المستخدمة لواجهة المستخدم. تسري التغييرات عند الولوج التالي.
add-language = إضافة لُغة
    .context = أضِف لُغة
install-additional-languages = ثبِّت لُغات إضافية
region = المنطقة

## Applications

applications = التطبيقات

## Applications: Default Applications

default-apps = التطبيقات المبدئية
    .desc = متصفح الويب المبدئي، عميل البريد، متصفح الملفات، والتطبيقات الأخرى.
    .web-browser = متصفح الويب
    .file-manager = مدير الملفات
    .mail-client = عميل البريد
    .music = صوتيات
    .video = فيديو
    .photos = صور
    .calendar = تقويم
    .terminal = طرفية
    .other-associations = جمعيات أخرى
    .text-editor = محرِّر نصوص

## Applications: Startup Applications

startup-apps = تطبيقات بدء التشغيل
    .desc = اضبط التطبيقات التي تعمل عند الولوج.
    .add = أضِف تطبيق
    .user = التطبيقات التي تُشغّل عند الولوج
    .none = لم تُضاف تطبيقات بدء تشغيل
    .remove-dialog-title = أزلِ { $name }؟
    .remove-dialog-description = أتريد إزالة تطبيق بدء التشغيل هذا؟
    .search-for-application = ابحث عن تطبيق

## Applications: Legacy Applications

legacy-applications = توافق تطبيقات اكس11
    .desc = تحجيم نطاق تطبيق نظام نافذة اكس11 والاختصارات العامة.
legacy-app-global-shortcuts = اختصارات عامة في تطبيقات اكس11
    .desc = تسمح الاختصارات العامة بالتعرّف على ضغطات المفاتيح وأحداث زر الفأرة المُجراة في تطبيقٍ ما لتطبيقاتٍ أخرى — لميزات مثل «اضغط لتتحدث» أو «اضغط لكتم الصوت». مبدئيًا، يُعطّل هذا في تطبيقات اكس11 لضمان عدم تمكن التطبيقات الأخرى من مراقبة أحداث لوحة المفاتيح والفأرة التي تحتوي على معلومات حساسة.
    .none = لا مفاتيح
    .modifiers = مفاتيح التعديل (Super وShift وControl وAlt)
    .combination = جميع المفاتيح أثناء الضغط على مفاتيح التعديل Super أو Control أو Alt
    .all = كل المفاتيح
    .mouse = أحداث زر الفأرة في تطبيقات اكس11
legacy-app-scaling = تحجيم نطاق تطبيقات نظام النوافذ اكس11
    .scaled-gaming = تحسين للألعاب وتطبيقات ملء الشاشة
    .gaming-description = قد تبدو تطبيقات اكس11 أكبر/أصغر قليلاً مقارنة بتطبيقات وايلاند.
    .scaled-applications = التحسين للتطبيقات
    .applications-description = قد لا تتطابق الألعاب وتطبيقات اكس11 بملء الشاشة مع دقة شاشتك.
    .scaled-compatibility = وضع التوافق الأقصى
    .compatibility-description = قد تبدو تطبيقات اكس11 ضبابية على شاشات HiDPI.
    .preferred-display = العرض المفضل للألعاب وتطبيقات اكس11 بملء الشاشة
    .no-display = لا شيء

## System

system = النظام والحسابات

## System: About

about = عَنْ
    .desc = اسم الجهاز، ومعلومات العتاد، ومبدئيات نظام التشغيل.
about-device = اسم الجهاز
    .desc = هذا الاسم يظهر للشبكات الأخرى وأجهزة بلوتوث.
about-hardware = العتاد
    .model = طراز العتاد
    .memory = الذاكرة
    .processor = المعالج
    .graphics = الرسوميات
    .disk-capacity = سعة القرص
about-os = نظام التشغيل
    .os = نظام التشغيل
    .os-architecture = معمارية نظام التشغيل
    .desktop-environment = بيئة سطح المكتب
    .windowing-system = نظام النوافذ
about-related = إعدادات متعلقة
    .support = احصل على الدعم

## System: Firmware

firmware = البرمجيات الثابتة
    .desc = تفاصيل البرمجيات الثابتة.

## System: Users

users = المستخدمون
    .desc = الاستيثاق وحسابات المستخدمين.
    .admin = مدير
    .standard = عادي
    .profile-add = اختر صورة ملف تعريف
administrator = مدير
    .desc = يمكن للمديرين تغيير الإعدادات لجميع المستخدمين، وإضافة مستخدمين آخرين وإزالتهم.
add-user = أضِف مستخدم
change-password = غيَّر كلمة السر
remove-user = أزِل المستخدم
full-name = الاسم الكامل
invalid-username = اسم مستخدم غير صالح.
password-mismatch = يجب أن تتطابق كلمة السر والتأكيد.
save = احفظ
never = أبدًا
keyboard-numlock-boot = قفل الأرقام
    .boot-state = الحالة عند الإقلاع
    .last-boot = آخر إقلاع
    .on = مفعَّل
    .off = معطَّل
    .set = عيِّن حالة إقلاع قفل الأرقام
