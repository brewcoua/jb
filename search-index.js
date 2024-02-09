var searchIndex = new Map(JSON.parse('[\
["jb",{"doc":"<code>JetBrains</code> CLI","t":"EEECQQQQQCCQQQCQQQCCQCCFFNNNNOONNNNNNNONNNNNNOOONNNNNNNNOFNNNNONNNHNONNNPPPPGPNNNNNNNNNNNNNNNNNNNFPPINNNNNNNNNNNNNNNNNNPPPGFPNNNNNNNNNNNNNNNNNNOONNNNNNNEEFEECNNNNNNCONNNNNNNNNNNNNNNNNNCONNNNNNNCONNNNNNNNCOKKKMMMMMMMMFNNNNNNNNNNNNNNNNNONONNONNNNNPPPPPPPPPPPPGPPPPPPPPPPNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNPPPGNNNNNNNNNNNNNNNNNNNNNNNNNFFNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNONOONNNNNNNNNNNNNNOHH","n":["Batch","Result","Tool","api","bail","bail_with","batch","batch_with","debug","env","error","error","info","info_elapsed","log","log","log_elapsed","make","tool","util","warn","deserial","fetch","Download","Release","borrow","borrow","borrow_mut","borrow_mut","build","checksum_link","clone","clone","clone_into","clone_into","deserialize","deserialize","download","downloads","fmt","fmt","from","from","into","into","link","release","size","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","version","Fetch","borrow","borrow_mut","clone","clone_into","download","fmt","from","into","release","to_owned","tool","try_from","try_into","type_id","BinariesDirectory","IconsDirectory","Notify","ToolsDirectory","Variable","Verbose","borrow","borrow_mut","clone","clone_into","default","env","fmt","from","get","get_bool","get_one","get_or","into","set","set_one","to_owned","try_from","try_into","type_id","Batch","Err","Ok","Result","add","borrow","borrow_mut","default","errors","extend","first","fmt","from","from","into","is_empty","len","new","to_string","try_from","try_into","type_id","Debug","Error","Info","Level","Logger","Warning","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","eq","equivalent","equivalent","equivalent","fmt","fmt","from","from","instance","into","into","log","name","started","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","Build","Kind","Tool","Type","Version","action","as_executable","as_icon","as_path","as_str","borrow","borrow_mut","build","build","clone","clone_into","cmp","compare","eq","equivalent","equivalent","equivalent","fill","fmt","fmt","from","from_kind","from_str","hash","into","is_installed","is_linked","kind","kind","link","list","list_kind","list_matching","matched","new","partial_cmp","release","release","sync","to_owned","to_string","try_from","try_into","type_id","unlink","unlink_with_alternative","version","version","Link","List","Probe","is_linked","link","list","list_kind","list_matching","sync","unlink","unlink_with_alternative","Build","borrow","borrow_mut","clone","clone_into","cmp","compare","deserialize","eq","equivalent","equivalent","equivalent","fmt","fmt","from","from_str","hash","into","major","matched","minor","new","partial_cmp","patch","to_owned","to_string","try_from","try_into","type_id","Aqua","CLion","CLionNova","DataGrip","DataSpell","DotMemory","DotTrace","Fleet","Gateway","GoLand","IntelliJIdeaCommunity","IntelliJIdeaUltimate","Kind","MPS","PhpStorm","PyCharmCommunity","PyCharmProfessional","Rider","RubyMine","RustRover","Space","WebStorm","Writerside","as_executable","as_icon","as_str","binary","borrow","borrow_mut","clone","clone_into","cmp","code","compare","eq","equivalent","equivalent","equivalent","fmt","fmt","from","from_str","hash","into","latest","linked","list","partial_cmp","to_owned","to_string","try_from","try_into","type_id","EAP","Preview","Release","Type","as_str","borrow","borrow_mut","clone","clone_into","cmp","compare","deserialize","eq","equivalent","equivalent","equivalent","fmt","fmt","from","from_str","hash","into","kind_default","partial_cmp","to_owned","to_string","try_from","try_into","type_id","Major","Version","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmp","cmp","compare","compare","deserialize","eq","eq","equivalent","equivalent","equivalent","equivalent","equivalent","equivalent","fmt","fmt","fmt","fmt","from","from","from_str","from_str","hash","hash","into","into","major","matched","minor","month","new","new","partial_cmp","partial_cmp","to_owned","to_owned","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","year","download_extract","strip_content"],"q":[[0,"jb"],[21,"jb::api"],[23,"jb::api::deserial"],[57,"jb::api::fetch"],[72,"jb::env"],[97,"jb::error"],[119,"jb::log"],[152,"jb::tool"],[205,"jb::tool::action"],[216,"jb::tool::build"],[245,"jb::tool::kind"],[298,"jb::tool::release"],[327,"jb::tool::version"],[381,"jb::util"],[383,"core::result"],[384,"serde::de"],[385,"anyhow"],[386,"core::fmt"],[387,"core::fmt"],[388,"alloc::string"],[389,"core::convert"],[390,"core::marker"],[391,"core::marker"],[392,"core::option"],[393,"std::thread"],[394,"dashmap::mapref::one"],[395,"core::fmt"],[396,"core::cmp"],[397,"core::hash"],[398,"indicatif::progress_bar"]],"d":["","","","Module for handling the API requests and responses.","Return a batch of errors directly from a format string","Return a batch of errors directly from an error with a …","Create a new batch of errors from a single error","Add an error to the given batch","","Module for handling defaults and environment variables.","Module for handling errors in a batch","","","","","","","","Module for tools and tool actions.","Module for utilities.","","Module for deserializing responses from <code>JetBrains</code>’ API.","Module for fetching releases from <code>JetBrains</code>’ API.","The deserialized download data from <code>JetBrains</code>’ API. …","The deserialized release data from <code>JetBrains</code>’ API.","","","","","","","","","","","","","Returns the download for the current platform and …","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","The fetched data for a tool.","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Fetches the latest release of a tool from <code>JetBrains</code>’ API.","","","","","","The directory where tool binaries are installed. (e.g. …","The directory where tool icons are installed. (e.g. …","Whether to enable notifications for long-running tasks.","The directory where tools are installed. (e.g. …","","Whether to enable verbose logging.","","","","","Get the default value for a variable.","Get the name of the environment variable.","","Returns the argument unchanged.","Get the value of the variable.","Get the value of the variable as a bool.","Get the value of a variable.","Get the value of a variable, or a default value if it is …","Calls <code>U::from(self)</code>.","Set the value of the variable.","Set the value of a variable.","","","","","A batch of errors that occurred while executing a command","Contains the error value","Contains the success value","A type alias for a result that can return a batch of errors","Add an error to the batch","","","","Get the errors in the batch","Extend the batch with another batch","Get the first error in the batch","","Create a new batch of errors from a single error","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Check if the batch is empty","Get the number of errors in the batch","Create a new batch of errors","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Get the logger for the current thread or create a new one …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Log a message.","","","","","","","","","","","","A tool.","","","Action module for tool.","Returns the path to the tool’s executable.","Returns the path to the tool’s icon.","Returns the path to the tool.","Returns the tool as a string.","","","Build version types and parsing","","","","","","","","","","Fills the tool with the latest version, build, and release.","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Returns whether the tool is installed.","","<code>JetBrains</code> tool kinds and parsing","","","","","","Returns whether the tool matches another tool.","","","","","","","","","","","","","Version types and parsing","","","","","Returns whether the tool is linked.","Links the tool.","Lists all tools.","Lists tools of a specific kind.","Lists tools that match the current tool.","Finds a compatible release for the tool.","Unlinks the tool.","Unlinks the tool and links an alternative version.","A build version number","","","","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","Returns whether the build version matches another build …","","","","","","","","","","","","","","","","","","","","","","The tool kind.","","","","","","","","","","","Get the relative path to the executable for this tool kind.","Get the relative path to the icon for this tool kind.","Get the tool kind as a string (e.g. “…","Get the binary name for this tool kind.","","","","","","Get the tool kind as a code (e.g. “IIU”, “IIC”).","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Get the latest tool of this kind.","Get the linked tool of this kind.","Returns a list of all tool kinds.","","","","","","","","","","","Returns the release type as a string.","","","","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Returns the default release type for the given kind.","","","","","","","A major version number (e.g. 2021.1)","A version number","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","The major version (e.g. 2021.1)","Returns whether the version matches another version.","The minor version (if any)","","","","","","","","","","","","","","","","","Download and extract a tarball from a URL.","Strip the content of a folder, moving all files and …"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,1,2,1,2,1,2,1,2,1,2,1,1,1,2,1,2,1,2,2,1,2,1,2,1,2,1,2,1,2,1,0,10,10,10,10,10,10,10,10,0,10,10,10,10,10,12,12,12,12,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,45,45,0,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,25,25,25,0,0,25,25,26,25,26,25,25,25,25,25,25,25,26,25,26,26,25,26,26,26,26,25,25,26,25,26,25,26,0,0,0,0,0,0,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,0,11,0,0,0,37,37,38,38,38,39,37,37,0,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,32,32,32,32,32,32,32,32,32,32,32,32,0,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,36,36,36,0,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,0,0,34,42,34,42,34,42,34,42,34,42,34,42,34,34,42,34,34,34,42,42,42,34,34,42,42,34,42,34,42,34,42,34,42,34,34,34,42,34,42,34,42,34,42,34,42,34,42,34,42,34,42,42,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,[1,1],[2,2],[[-1,-2],3,[],[]],[[-1,-2],3,[],[]],[-1,[[4,[1]]],5],[-1,[[4,[2]]],5],[1,[[6,[2]]]],0,[[1,7],8],[[2,7],8],[-1,-1,[]],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[10,10],[[-1,-2],3,[],[]],0,[[10,7],8],[-1,-1,[]],[-1,-2,[],[]],[11,[[6,[10]]]],[-1,-2,[],[]],0,[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[12,12],[[-1,-2],3,[],[]],[12,-1,[[14,[13]],15,16]],[12,17],[[12,7],8],[-1,-1,[]],[12,-1,[[14,[13]],15,16]],[12,18],[12,-1,[[14,[13]],15,16]],[[12,-1],-1,[[14,[13]],15,16]],[-1,-2,[],[]],[[12,-1],3,[[19,[13]]]],[[12,-1],3,[[19,[13]]]],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],0,0,0,0,[[20,21],3],[-1,-2,[],[]],[-1,-2,[],[]],[[],20],[20,[[22,[21]]]],[[20,20],3],[20,[[23,[21]]]],[[20,7],8],[21,20],[-1,-1,[]],[-1,-2,[],[]],[20,18],[20,24],[[],20],[-1,13,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[25,25],[[-1,-2],3,[],[]],[[25,25],18],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[25,7],8],[[26,7],8],[-1,-1,[]],[-1,-1,[]],[[],[[28,[27,26]]]],[-1,-2,[],[]],[-1,-2,[],[]],[[26,25,-1,18],3,29],0,0,[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],0,0,0,0,0,0,[11,30],[11,30],[11,30],[11,13],[-1,-2,[],[]],[-1,-2,[],[]],0,0,[11,11],[[-1,-2],3,[],[]],[[11,11],31],[[-1,-2],31,[],[]],[[11,11],18],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[11,[[6,[11]]]],[[11,7],8],[[11,7],8],[-1,-1,[]],[32,11],[17,[[4,[11,-1]]],[]],[[11,-1],3,33],[-1,-2,[],[]],[11,18],[11,18],0,0,[11,[[6,[3]]]],[[],[[6,[[22,[11]]]]]],[32,[[6,[[22,[11]]]]]],[11,[[6,[[22,[11]]]]]],[[11,11],18],[[32,[23,[34]],[23,[35]],[23,[36]]],11],[[11,11],[[23,[31]]]],0,0,[11,[[6,[2]]]],[-1,-2,[],[]],[-1,13,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[11,[[6,[3]]]],[11,[[6,[3]]]],0,0,0,0,0,[37,18],[37,[[6,[3]]]],[[],[[6,[[22,[11]]]]]],[32,[[6,[[22,[11]]]]]],[38,[[6,[[22,[11]]]]]],[39,[[6,[2]]]],[37,[[6,[3]]]],[37,[[6,[3]]]],0,[-1,-2,[],[]],[-1,-2,[],[]],[35,35],[[-1,-2],3,[],[]],[[35,35],31],[[-1,-2],31,[],[]],[-1,[[4,[35]]],5],[[35,35],18],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[35,7],8],[[35,7],8],[-1,-1,[]],[17,[[4,[35,-1]]],[]],[[35,-1],3,33],[-1,-2,[],[]],0,[[35,35],18],0,[[40,40,[23,[40]]],35],[[35,35],[[23,[31]]]],0,[-1,-2,[],[]],[-1,13,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[32,13],[32,13],[32,17],[32,17],[-1,-2,[],[]],[-1,-2,[],[]],[32,32],[[-1,-2],3,[],[]],[[32,32],31],[32,17],[[-1,-2],31,[],[]],[[32,32],18],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[32,7],8],[[32,7],8],[-1,-1,[]],[17,[[4,[32,-1]]],[]],[[32,-1],3,33],[-1,-2,[],[]],[32,[[6,[[23,[11]]]]]],[32,[[6,[[23,[11]]]]]],[[],[[41,[32]]]],[[32,32],[[23,[31]]]],[-1,-2,[],[]],[-1,13,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],0,0,0,0,[36,17],[-1,-2,[],[]],[-1,-2,[],[]],[36,36],[[-1,-2],3,[],[]],[[36,36],31],[[-1,-2],31,[],[]],[-1,[[4,[36]]],5],[[36,36],18],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[36,7],8],[[36,7],8],[-1,-1,[]],[17,[[4,[36,-1]]],[]],[[36,-1],3,33],[-1,-2,[],[]],[32,36],[[36,36],[[23,[31]]]],[-1,-2,[],[]],[-1,13,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[34,34],[42,42],[[-1,-2],3,[],[]],[[-1,-2],3,[],[]],[[34,34],31],[[42,42],31],[[-1,-2],31,[],[]],[[-1,-2],31,[],[]],[-1,[[4,[34]]],5],[[34,34],18],[[42,42],18],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[-1,-2],18,[],[]],[[34,7],8],[[34,7],8],[[42,7],8],[[42,7],8],[-1,-1,[]],[-1,-1,[]],[17,[[4,[34,-1]]],[]],[17,[[4,[42,-1]]],[]],[[34,-1],3,33],[[42,-1],3,33],[-1,-2,[],[]],[-1,-2,[],[]],0,[[34,34],18],0,0,[[42,[23,[43]]],34],[[40,43],42],[[34,34],[[23,[31]]]],[[42,42],[[23,[31]]]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,13,[]],[-1,13,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],0,[[17,30,[23,[17]],[23,[44]]],[[6,[3]]]],[30,[[6,[3]]]]],"c":[],"p":[[5,"Release",23],[5,"Download",23],[1,"tuple"],[6,"Result",383],[10,"Deserializer",384],[8,"Result",385],[5,"Formatter",386],[8,"Result",386],[5,"TypeId",387],[5,"Fetch",57],[5,"Tool",152],[6,"Variable",72],[5,"String",388],[10,"From",389],[10,"Send",390],[10,"Sync",390],[1,"str"],[1,"bool"],[10,"Into",389],[5,"Batch",97],[5,"Error",385],[5,"Vec",391],[6,"Option",392],[1,"usize"],[6,"Level",119],[5,"Logger",119],[5,"ThreadId",393],[5,"RefMut",394],[10,"Display",386],[5,"PathBuf",395],[6,"Ordering",396],[6,"Kind",245],[10,"Hasher",397],[5,"Version",327],[5,"Build",216],[6,"Type",298],[10,"Link",205],[10,"List",205],[10,"Probe",205],[1,"u16"],[1,"slice"],[5,"Major",327],[1,"u8"],[5,"ProgressBar",398],[8,"Result",97]],"b":[[175,"impl-Display-for-Tool"],[176,"impl-Debug-for-Tool"],[228,"impl-Debug-for-Build"],[229,"impl-Display-for-Build"],[283,"impl-Debug-for-Kind"],[284,"impl-Display-for-Kind"],[314,"impl-Debug-for-Type"],[315,"impl-Display-for-Type"],[350,"impl-Display-for-Version"],[351,"impl-Debug-for-Version"],[352,"impl-Display-for-Major"],[353,"impl-Debug-for-Major"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
