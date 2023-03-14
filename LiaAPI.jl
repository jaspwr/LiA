using Random
using Plots

struct LiaDocInfo
    initialsed::Bool
    file_name::String
    output_folder_canonical::String
    image_folder_canonical::String
end


function embedfig(fig::Plots.Plot, width::String)
    if !doc_info.initialsed
        error("Julia session not initialised for this document.");
    end
    file_name = randstring(12) * ".png";
    img_output_path = joinpath(doc_info.image_folder_canonical, file_name);
    savefig(fig, img_output_path);
    # TODO: Make this work with other formats and LaTeX image packages
    return "\\includegraphics[width=" * width * "]{" * file_name * "}";
end
